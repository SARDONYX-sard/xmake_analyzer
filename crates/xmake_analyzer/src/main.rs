//! Language server for xmake build scripts (`xmake.lua`).
//!
//! Speaks the Language Server Protocol over stdio. Logs go to stderr and are
//! controlled with the `RUST_LOG` environment variable (default: `info`).

mod config;
mod conv;
mod fmt;
mod handlers;
mod lint;
mod probe;
mod scan;
mod server;

use lsp_types::{
    CompletionOptions, FoldingRangeProviderCapability, HoverProviderCapability, InitializeParams,
    InitializeResult, OneOf, RenameOptions, ServerCapabilities, ServerInfo,
    TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions,
};
use xmake_ide::{TokenModifier, TokenType};

const USAGE: &str = "\
xmake_analyzer: a language server for xmake build scripts.

USAGE:
    xmake_analyzer            Run the server over stdio.
    xmake_analyzer fmt ...    Format files, see `xmake_analyzer fmt --help`.
    xmake_analyzer lint ...   Check projects, see `xmake_analyzer lint --help`.
    xmake_analyzer --version  Print the version.
    xmake_analyzer --help     Print this help.

Settings are read from the `initializationOptions` of the client, see the
README. Logs are written to stderr; set RUST_LOG=debug for more.";

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("fmt") => std::process::exit(fmt::run(&args[1..])),
        Some("lint") => std::process::exit(lint::run(&args[1..])),
        _ => {}
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("xmake_analyzer {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{USAGE}");
        return Ok(());
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stderr)
        .init();
    log::info!("xmake_analyzer {} starting", env!("CARGO_PKG_VERSION"));

    let (connection, io_threads) = lsp_server::Connection::stdio();
    let (id, params) = connection.initialize_start()?;
    let init: InitializeParams = serde_json::from_value(params)?;
    let result = InitializeResult {
        capabilities: capabilities(),
        server_info: Some(ServerInfo {
            name: "xmake_analyzer".to_owned(),
            version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        }),
    };
    connection.initialize_finish(id, serde_json::to_value(result)?)?;

    server::Server::new(connection.sender.clone(), &init).run(&connection)?;
    drop(connection);
    io_threads.join()?;
    log::info!("xmake_analyzer stopped");
    Ok(())
}

fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        completion_provider: Some(CompletionOptions {
            // `.`/`:` for members, quotes for values, `/` for paths.
            trigger_characters: Some(
                [".", ":", "\"", "'", "/", "@", "|", "$"].map(str::to_owned).to_vec(),
            ),
            ..Default::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        declaration_provider: Some(lsp_types::DeclarationCapability::Simple(true)),
        signature_help_provider: Some(lsp_types::SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_owned(), ",".to_owned()]),
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        inlay_hint_provider: Some(OneOf::Left(true)),
        semantic_tokens_provider: Some(
            lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
                lsp_types::SemanticTokensOptions {
                    legend: lsp_types::SemanticTokensLegend {
                        token_types: TokenType::ALL.iter().map(|(_, n)| (*n).into()).collect(),
                        token_modifiers: TokenModifier::ALL
                            .iter()
                            .map(|(_, n)| (*n).into())
                            .collect(),
                    },
                    full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
                    range: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                },
            ),
        ),
        code_action_provider: Some(lsp_types::CodeActionProviderCapability::Options(
            lsp_types::CodeActionOptions {
                code_action_kinds: Some(vec![lsp_types::CodeActionKind::QUICKFIX]),
                ..Default::default()
            },
        )),
        code_lens_provider: Some(lsp_types::CodeLensOptions { resolve_provider: Some(false) }),
        document_formatting_provider: Some(OneOf::Left(true)),
        ..Default::default()
    }
}
