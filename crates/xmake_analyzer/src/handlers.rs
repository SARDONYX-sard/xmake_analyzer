//! Request handlers: protocol parameters in, IDE query, protocol result out.
//!
//! The IDE layer returns borrowed data; this is where it becomes the owned
//! strings the protocol types need.

// `lsp_types::Uri` keys: its hash only depends on the URI text.
#![allow(clippy::mutable_key_type)]

use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, CompletionTextEdit,
    Diagnostic, DiagnosticSeverity, DocumentHighlight, DocumentHighlightParams, DocumentSymbol,
    DocumentSymbolParams, DocumentSymbolResponse, FoldingRange, FoldingRangeKind,
    FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents,
    HoverParams, InsertTextFormat, Location, MarkupContent, MarkupKind, NumberOrString,
    PrepareRenameResponse, ReferenceParams, RenameParams, SymbolInformation, SymbolKind,
    TextDocumentPositionParams, TextEdit, WorkspaceEdit, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};
use rustc_hash::FxHashMap;
use xmake_ide::{AnalyzedFile, NavTarget, Severity, xmake_api::Scope};

use crate::{conv, server::Server};

fn markdown(value: String) -> MarkupContent {
    MarkupContent { kind: MarkupKind::Markdown, value }
}

pub(crate) fn diagnostic(file: &AnalyzedFile, d: &xmake_ide::Diagnostic<'_>) -> Diagnostic {
    let severity = match d.severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
        Severity::Information => DiagnosticSeverity::INFORMATION,
        Severity::Hint => DiagnosticSeverity::HINT,
    };
    Diagnostic {
        range: conv::range(file, d.range),
        severity: Some(severity),
        code: Some(NumberOrString::String(d.code.to_owned())),
        source: Some("xmake-analyzer".to_owned()),
        message: d.message.clone().into_owned(),
        tags: match d.code {
            "unused-local" | "unused-require" => Some(vec![lsp_types::DiagnosticTag::UNNECESSARY]),
            "deprecated-api" => Some(vec![lsp_types::DiagnosticTag::DEPRECATED]),
            _ => None,
        },
        ..Default::default()
    }
}

pub(crate) fn completion(server: &Server, params: CompletionParams) -> Option<CompletionResponse> {
    let position = params.text_document_position;
    let (path, file) = server.file(&position.text_document.uri)?;
    let offset = conv::offset(file, position.position);
    let items = xmake_ide::completions(&server.project, file, Some(&path), offset);
    let items = items
        .into_iter()
        .map(|item| {
            use xmake_ide::CompletionKind as K;
            let kind = match item.kind {
                K::Function => CompletionItemKind::FUNCTION,
                K::Method => CompletionItemKind::METHOD,
                K::Module => CompletionItemKind::MODULE,
                K::Variable => CompletionItemKind::VARIABLE,
                K::Keyword => CompletionItemKind::KEYWORD,
                K::Snippet => CompletionItemKind::SNIPPET,
                K::Value => CompletionItemKind::ENUM_MEMBER,
                K::Scope => CompletionItemKind::STRUCT,
                K::File => CompletionItemKind::FILE,
                K::Folder => CompletionItemKind::FOLDER,
            };
            let snippet = item.snippet.filter(|_| server.snippets);
            let (new_text, format) = match snippet {
                Some(snippet) => (snippet.into_owned(), InsertTextFormat::SNIPPET),
                None => (item.label.to_string(), InsertTextFormat::PLAIN_TEXT),
            };
            let edit = TextEdit { range: conv::range(file, item.replace), new_text };
            CompletionItem {
                sort_text: Some(format!("{}{}", item.group, item.label)),
                label: item.label.into_owned(),
                kind: Some(kind),
                detail: item.detail.map(|d| d.into_owned()),
                documentation: item
                    .documentation
                    .map(|d| lsp_types::Documentation::MarkupContent(markdown(d.into_owned()))),
                insert_text_format: Some(format),
                text_edit: Some(CompletionTextEdit::Edit(edit)),
                ..Default::default()
            }
        })
        .collect();
    Some(CompletionResponse::Array(items))
}

pub(crate) fn hover(server: &Server, params: HoverParams) -> Option<Hover> {
    let position = params.text_document_position_params;
    let (_, file) = server.file(&position.text_document.uri)?;
    let offset = conv::offset(file, position.position);
    let hover = xmake_ide::hover(&server.project, file, offset)?;
    Some(Hover {
        contents: HoverContents::Markup(markdown(hover.markdown)),
        range: Some(conv::range(file, hover.range)),
    })
}

/// Converts targets, which may be in files other than the request's.
fn locations(server: &Server, targets: Vec<NavTarget<'_>>) -> Vec<Location> {
    targets
        .into_iter()
        .map(|t| {
            let range = match server.project.file(&t.path) {
                Some(file) => conv::range(file, t.range),
                // A file that is not loaded: its start.
                None => lsp_types::Range::default(),
            };
            Location { uri: conv::path_to_uri(&t.path), range }
        })
        .collect()
}

pub(crate) fn definition(
    server: &Server,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let TextDocumentPositionParams { text_document, position } =
        params.text_document_position_params;
    let (path, file) = server.file(&text_document.uri)?;
    let offset = conv::offset(file, position);
    let targets = xmake_ide::goto_definition(&server.project, file, &path, offset);
    Some(GotoDefinitionResponse::Array(locations(server, targets)))
}

pub(crate) fn references(server: &Server, params: ReferenceParams) -> Option<Vec<Location>> {
    let TextDocumentPositionParams { text_document, position } = params.text_document_position;
    let (path, file) = server.file(&text_document.uri)?;
    let offset = conv::offset(file, position);
    let mut targets = xmake_ide::references(&server.project, file, &path, offset);
    if !params.context.include_declaration {
        let definitions = xmake_ide::goto_definition(&server.project, file, &path, offset);
        targets.retain(|t| !definitions.contains(t));
    }
    Some(locations(server, targets))
}

pub(crate) fn highlights(
    server: &Server,
    params: DocumentHighlightParams,
) -> Option<Vec<DocumentHighlight>> {
    let TextDocumentPositionParams { text_document, position } =
        params.text_document_position_params;
    let (_, file) = server.file(&text_document.uri)?;
    let offset = conv::offset(file, position);
    let ranges = xmake_ide::highlights(file, offset);
    Some(
        ranges
            .into_iter()
            .map(|r| DocumentHighlight { range: conv::range(file, r), kind: None })
            .collect(),
    )
}

fn symbol_kind(kind: xmake_ide::SymbolKind) -> SymbolKind {
    match kind {
        xmake_ide::SymbolKind::Scope(Scope::Target) => SymbolKind::CLASS,
        xmake_ide::SymbolKind::Scope(Scope::Option) => SymbolKind::PROPERTY,
        xmake_ide::SymbolKind::Scope(Scope::Rule) => SymbolKind::INTERFACE,
        xmake_ide::SymbolKind::Scope(Scope::Package) => SymbolKind::PACKAGE,
        xmake_ide::SymbolKind::Scope(Scope::Toolchain) => SymbolKind::OBJECT,
        xmake_ide::SymbolKind::Scope(Scope::Task) => SymbolKind::EVENT,
        xmake_ide::SymbolKind::Scope(Scope::Root) => SymbolKind::NAMESPACE,
        xmake_ide::SymbolKind::Function => SymbolKind::FUNCTION,
    }
}

fn symbol_detail(kind: xmake_ide::SymbolKind) -> Option<String> {
    match kind {
        xmake_ide::SymbolKind::Scope(scope) => Some(scope.name().to_owned()),
        xmake_ide::SymbolKind::Function => None,
    }
}

pub(crate) fn document_symbols(
    server: &Server,
    params: DocumentSymbolParams,
) -> Option<DocumentSymbolResponse> {
    let (_, file) = server.file(&params.text_document.uri)?;
    let symbols = xmake_ide::document_symbols(file)
        .into_iter()
        .map(|s| {
            #[allow(deprecated)]
            DocumentSymbol {
                name: s.name.to_owned(),
                detail: symbol_detail(s.kind),
                kind: symbol_kind(s.kind),
                tags: None,
                deprecated: None,
                range: conv::range(file, s.range),
                selection_range: conv::range(file, s.selection),
                children: None,
            }
        })
        .collect();
    Some(DocumentSymbolResponse::Nested(symbols))
}

pub(crate) fn workspace_symbols(
    server: &Server,
    params: WorkspaceSymbolParams,
) -> Option<WorkspaceSymbolResponse> {
    let symbols = xmake_ide::workspace_symbols(&server.project, &params.query)
        .into_iter()
        .filter_map(|(path, s)| {
            let file = server.project.file(path)?;
            #[allow(deprecated)]
            Some(SymbolInformation {
                name: s.name.to_owned(),
                kind: symbol_kind(s.kind),
                tags: None,
                deprecated: None,
                location: Location {
                    uri: conv::path_to_uri(path),
                    range: conv::range(file, s.selection),
                },
                container_name: symbol_detail(s.kind),
            })
        })
        .collect();
    Some(WorkspaceSymbolResponse::Flat(symbols))
}

pub(crate) fn prepare_rename(
    server: &Server,
    params: TextDocumentPositionParams,
) -> Option<PrepareRenameResponse> {
    let (_, file) = server.file(&params.text_document.uri)?;
    let offset = conv::offset(file, params.position);
    let range = xmake_ide::prepare_rename(file, offset)?;
    Some(PrepareRenameResponse::Range(conv::range(file, range)))
}

pub(crate) fn rename(server: &Server, params: RenameParams) -> Option<WorkspaceEdit> {
    let TextDocumentPositionParams { text_document, position } = params.text_document_position;
    let (path, file) = server.file(&text_document.uri)?;
    let offset = conv::offset(file, position);
    let targets = xmake_ide::rename(&server.project, file, &path, offset);
    if targets.is_empty() {
        return None;
    }
    let mut changes: FxHashMap<lsp_types::Uri, Vec<TextEdit>> = FxHashMap::default();
    for target in targets {
        let Some(target_file) = server.project.file(&target.path) else {
            continue;
        };
        let edit = TextEdit {
            range: conv::range(target_file, target.range),
            new_text: params.new_name.clone(),
        };
        changes.entry(conv::path_to_uri(&target.path)).or_default().push(edit);
    }
    #[allow(clippy::disallowed_types)]
    let changes: std::collections::HashMap<_, _> = changes.into_iter().collect();
    Some(WorkspaceEdit { changes: Some(changes), ..Default::default() })
}

pub(crate) fn folding_ranges(
    server: &Server,
    params: FoldingRangeParams,
) -> Option<Vec<FoldingRange>> {
    let (_, file) = server.file(&params.text_document.uri)?;
    let ranges = xmake_ide::folding_ranges(file)
        .into_iter()
        .filter_map(|fold| {
            let range = conv::range(file, fold.range);
            (range.end.line > range.start.line).then(|| FoldingRange {
                start_line: range.start.line,
                end_line: range.end.line,
                kind: fold.is_comment.then_some(FoldingRangeKind::Comment),
                ..Default::default()
            })
        })
        .collect();
    Some(ranges)
}

pub(crate) fn signature_help(
    server: &Server,
    params: lsp_types::SignatureHelpParams,
) -> Option<lsp_types::SignatureHelp> {
    let TextDocumentPositionParams { text_document, position } =
        params.text_document_position_params;
    let (_, file) = server.file(&text_document.uri)?;
    let offset = conv::offset(file, position);
    let help = xmake_ide::signature_help(&server.project, file, offset)?;
    let parameters = help
        .params
        .iter()
        .enumerate()
        .map(|(i, &(start, end))| {
            let offsets =
                [utf16_len(&help.label[..start as usize]), utf16_len(&help.label[..end as usize])];
            let doc =
                if i as u32 == help.active_param { help.active_param_doc.clone() } else { None };
            lsp_types::ParameterInformation {
                label: lsp_types::ParameterLabel::LabelOffsets(offsets),
                documentation: doc.map(|d| lsp_types::Documentation::MarkupContent(markdown(d))),
            }
        })
        .collect();
    let signature = lsp_types::SignatureInformation {
        label: help.label,
        documentation: help
            .documentation
            .map(|d| lsp_types::Documentation::MarkupContent(markdown(d.into_owned()))),
        parameters: Some(parameters),
        active_parameter: Some(help.active_param),
    };
    Some(lsp_types::SignatureHelp {
        signatures: vec![signature],
        active_signature: Some(0),
        active_parameter: Some(help.active_param),
    })
}

fn utf16_len(s: &str) -> u32 {
    s.encode_utf16().count() as u32
}

pub(crate) fn inlay_hints(
    server: &Server,
    params: lsp_types::InlayHintParams,
) -> Option<Vec<lsp_types::InlayHint>> {
    use xmake_ide::InlayKind;
    let (_, file) = server.file(&params.text_document.uri)?;
    let range = xmake_ide::TextRange::new(
        conv::offset(file, params.range.start),
        conv::offset(file, params.range.end),
    );
    let hints = xmake_ide::inlay_hints(&server.project, file, Some(range))
        .into_iter()
        .map(|hint| {
            let (label, kind) = match hint.kind {
                InlayKind::Type => {
                    (format!(": {}", hint.label), Some(lsp_types::InlayHintKind::TYPE))
                }
                InlayKind::Parameter => {
                    (format!("{}:", hint.label), Some(lsp_types::InlayHintKind::PARAMETER))
                }
                InlayKind::End => (format!("-- {}", hint.label), None),
                InlayKind::Return => {
                    (format!("-> {}", hint.label), Some(lsp_types::InlayHintKind::TYPE))
                }
            };
            lsp_types::InlayHint {
                position: conv::position(file, hint.offset),
                label: lsp_types::InlayHintLabel::String(label),
                kind,
                text_edits: None,
                tooltip: None,
                padding_left: Some(matches!(hint.kind, InlayKind::End | InlayKind::Return)),
                padding_right: Some(hint.kind == InlayKind::Parameter),
                data: None,
            }
        })
        .collect();
    Some(hints)
}

/// Encodes tokens relative to each other, columns in UTF-16 units.
fn encode_semantic_tokens(
    file: &AnalyzedFile,
    tokens: &[xmake_ide::SemanticToken],
) -> Vec<lsp_types::SemanticToken> {
    let mut out = Vec::with_capacity(tokens.len());
    let (mut prev_line, mut prev_col) = (0, 0);
    for token in tokens {
        let start = conv::position(file, token.range.start());
        let end = conv::position(file, token.range.end());
        let delta_line = start.line - prev_line;
        let delta_start =
            if delta_line == 0 { start.character - prev_col } else { start.character };
        out.push(lsp_types::SemanticToken {
            delta_line,
            delta_start,
            length: end.character - start.character,
            token_type: token.ty as u32,
            token_modifiers_bitset: token.modifiers,
        });
        (prev_line, prev_col) = (start.line, start.character);
    }
    out
}

pub(crate) fn semantic_tokens_full(
    server: &Server,
    params: lsp_types::SemanticTokensParams,
) -> Option<lsp_types::SemanticTokensResult> {
    let (_, file) = server.file(&params.text_document.uri)?;
    let tokens = xmake_ide::semantic_tokens(&server.project, file, None);
    let data = encode_semantic_tokens(file, &tokens);
    Some(lsp_types::SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
        result_id: None,
        data,
    }))
}

pub(crate) fn semantic_tokens_range(
    server: &Server,
    params: lsp_types::SemanticTokensRangeParams,
) -> Option<lsp_types::SemanticTokensRangeResult> {
    let (_, file) = server.file(&params.text_document.uri)?;
    let range = xmake_ide::TextRange::new(
        conv::offset(file, params.range.start),
        conv::offset(file, params.range.end),
    );
    let tokens = xmake_ide::semantic_tokens(&server.project, file, Some(range));
    let data = encode_semantic_tokens(file, &tokens);
    Some(lsp_types::SemanticTokensRangeResult::Tokens(lsp_types::SemanticTokens {
        result_id: None,
        data,
    }))
}

/// Quick fixes of the diagnostics in the requested range.
pub(crate) fn code_actions(
    server: &Server,
    params: lsp_types::CodeActionParams,
) -> Option<lsp_types::CodeActionResponse> {
    let (_, file) = server.file(&params.text_document.uri)?;
    let range = xmake_ide::TextRange::new(
        conv::offset(file, params.range.start),
        conv::offset(file, params.range.end),
    );
    let mut actions = Vec::new();
    for found in xmake_ide::diagnostics(&server.project, file, &server.config.lints) {
        if found.range.intersect(range).is_none() {
            continue;
        }
        let allow = xmake_ide::allow_fix(file, &found);
        for (i, fix) in found.fixes.iter().chain(allow.as_ref()).enumerate() {
            let edit = TextEdit {
                range: conv::range(file, fix.range),
                new_text: fix.replacement.clone().into_owned(),
            };
            #[allow(clippy::disallowed_types)]
            let changes =
                std::collections::HashMap::from([(params.text_document.uri.clone(), vec![edit])]);
            actions.push(lsp_types::CodeActionOrCommand::CodeAction(lsp_types::CodeAction {
                title: fix.title.clone().into_owned(),
                kind: Some(lsp_types::CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic(file, &found)]),
                edit: Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }),
                // The first fix of a diagnostic is its best one.
                is_preferred: Some(i == 0 && !found.fixes.is_empty()),
                ..Default::default()
            }));
        }
    }
    Some(actions)
}

/// "N references" above each target, option, rule, ...; "▶ Run" above the
/// tasks runnable from the command line, when the client can run them.
pub(crate) fn code_lenses(
    server: &Server,
    params: lsp_types::CodeLensParams,
) -> Option<Vec<lsp_types::CodeLens>> {
    let (path, file) = server.file(&params.text_document.uri)?;
    let uri = &params.text_document.uri;
    let tasks =
        if server.config.run_task_lens { xmake_ide::runnable_tasks(file) } else { Vec::new() };
    let run_lenses = tasks.into_iter().map(|(range, name)| {
        let arguments =
            vec![serde_json::Value::from(name), serde_json::to_value(uri).unwrap_or_default()];
        lsp_types::CodeLens {
            range: conv::range(file, range),
            command: Some(lsp_types::Command {
                title: format!("▶ Run xmake {name}"),
                command: "xmake-analyzer.runTask".to_owned(),
                arguments: Some(arguments),
            }),
            data: None,
        }
    });
    let definitions = if server.config.references_lens {
        xmake_ide::code_lenses(&server.project, file, &path)
    } else {
        Vec::new()
    };
    let reference_lenses = definitions.into_iter().map(|(range, uses)| {
        let range = conv::range(file, range);
        let count = uses.len();
        let title =
            if count == 1 { "1 reference".to_owned() } else { format!("{count} references") };
        let arguments = vec![
            serde_json::to_value(&params.text_document.uri).unwrap_or_default(),
            serde_json::to_value(range.start).unwrap_or_default(),
            serde_json::to_value(locations(server, uses)).unwrap_or_default(),
        ];
        // The client converts the protocol values for
        // `editor.action.showReferences`, which takes editor ones.
        let command = "xmake-analyzer.showReferences".to_owned();
        lsp_types::CodeLens {
            range,
            command: Some(lsp_types::Command { title, command, arguments: Some(arguments) }),
            data: None,
        }
    });
    Some(run_lenses.chain(reference_lenses).collect())
}

/// Formats a whole file: editor options first, then `.editorconfig`.
pub(crate) fn formatting(
    server: &Server,
    params: lsp_types::DocumentFormattingParams,
) -> Option<Vec<TextEdit>> {
    let (path, file) = server.file(&params.text_document.uri)?;
    let editor = &params.options;
    let settings = &server.config.format;
    let options = xmake_fmt::FormatOptions::for_file(&path, |o| {
        o.indent = if editor.insert_spaces {
            xmake_fmt::Indent::Spaces(editor.tab_size.clamp(1, 16) as u8)
        } else {
            xmake_fmt::Indent::Tabs(1)
        };
        if let Some(final_newline) = editor.insert_final_newline {
            o.final_newline = final_newline;
        }
        o.indent_scopes = settings.indent_scopes && file.kind == xmake_ide::FileKind::Description;
        o.max_blank_lines = settings.max_blank_lines;
    });
    match xmake_fmt::format(&file.text, file.dialect, options) {
        Ok(formatted) if formatted == *file.text => Some(Vec::new()),
        Ok(formatted) => {
            let whole = xmake_ide::TextRange::up_to(xmake_ide::TextSize::of(&*file.text));
            Some(vec![TextEdit { range: conv::range(file, whole), new_text: formatted }])
        }
        Err(err) => {
            log::info!("{}: not formatted: {err}", path.display());
            None
        }
    }
}
