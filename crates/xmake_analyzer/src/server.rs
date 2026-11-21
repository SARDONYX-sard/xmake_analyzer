//! The server state and main loop.
//!
//! Requests are answered on the main thread: a query on a description file
//! takes microseconds. Two jobs run in the background and report through a
//! channel: scanning the workspace for `xmake.lua` files, and asking the
//! installed xmake for its API lists ([`crate::probe`]).

use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    sync::Arc,
};

use crossbeam_channel::{Receiver, Sender, select};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, Response};
use lsp_types::{
    DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, FileChangeType, InitializeParams,
    PublishDiagnosticsParams,
};
use rustc_hash::FxHashMap;
use serde::{Serialize, de::DeserializeOwned};
use xmake_ide::{
    LineCol, LineIndex, Project,
    xmake_api::ApiDb,
    xmake_syntax::{Dialect, LuaVersion},
};

use crate::{
    config::{Config, LuaSetting},
    conv, handlers,
    probe::{self, Probe},
    scan,
};

/// Results of the background jobs.
enum Event {
    Probed(Result<Box<Probe>, String>),
    Scanned(Vec<(PathBuf, String)>),
    /// The packages of the repositories, with the local repositories
    /// (`add_repositories`) indexed.
    Packages(Arc<xmake_ide::PackageIndex>, Vec<PathBuf>),
}

/// `xmake-analyzer/includedFiles`: the URIs of the `.lua` files loaded by
/// `includes()`.
#[derive(Serialize)]
struct IncludedFiles {
    uris: Vec<String>,
}

struct OpenDoc {
    uri: lsp_types::Uri,
    version: i32,
}

pub(crate) struct Server {
    sender: Sender<Message>,
    pub(crate) project: Project,
    /// Open documents: their URI as the client spells it, and version.
    /// Their text is in `project`.
    open: FxHashMap<PathBuf, OpenDoc>,
    roots: Vec<PathBuf>,
    pub(crate) config: Config,
    /// Whether the client accepts snippets in completions.
    pub(crate) snippets: bool,
    watch_files: bool,
    /// The Lua runtime of the installed xmake, once probed.
    luajit: Option<bool>,
    /// The local repositories (`add_repositories`) of the package index
    /// asked for last.
    repositories: Option<Vec<PathBuf>>,
    /// The included files and imported modules the client was told last.
    xmake_files: Option<Vec<String>>,
    events_tx: Sender<Event>,
    events_rx: Receiver<Event>,
    next_request_id: i32,
}

impl Server {
    pub(crate) fn new(sender: Sender<Message>, init: &InitializeParams) -> Server {
        let mut roots: Vec<PathBuf> = init
            .workspace_folders
            .iter()
            .flatten()
            .filter_map(|f| conv::uri_to_path(&f.uri))
            .collect();
        #[allow(deprecated)]
        if roots.is_empty() {
            roots.extend(init.root_uri.as_ref().and_then(conv::uri_to_path));
        }
        let text_caps = init.capabilities.text_document.as_ref();
        let snippets = text_caps
            .and_then(|t| t.completion.as_ref())
            .and_then(|c| c.completion_item.as_ref())
            .and_then(|i| i.snippet_support)
            .unwrap_or(false);
        let watch_files = (init.capabilities.workspace.as_ref())
            .and_then(|w| w.did_change_watched_files.as_ref())
            .and_then(|d| d.dynamic_registration)
            .unwrap_or(false);
        let config = Config::from_json(init.initialization_options.as_ref());
        let (events_tx, events_rx) = crossbeam_channel::unbounded();
        let mut server = Server {
            sender,
            project: Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT),
            open: FxHashMap::default(),
            roots,
            config,
            snippets,
            watch_files,
            luajit: None,
            repositories: None,
            xmake_files: None,
            events_tx,
            events_rx,
            next_request_id: 0,
        };
        server.project.dialect = server.dialect();
        server
    }

    /// The dialect from the settings and the probed runtime.
    fn dialect(&self) -> Dialect {
        let lua = match self.config.lua {
            LuaSetting::Fixed(lua) => lua,
            LuaSetting::Auto if self.luajit == Some(true) => LuaVersion::LuaJit,
            LuaSetting::Auto => LuaVersion::Lua54,
        };
        Dialect { lua }
    }

    pub(crate) fn run(mut self, connection: &Connection) -> anyhow::Result<()> {
        // `Connection::initialize_finish` consumed the `initialized`
        // notification already.
        self.on_initialized();
        loop {
            select! {
                recv(connection.receiver) -> msg => {
                    let Ok(msg) = msg else { return Ok(()) };
                    match msg {
                        Message::Request(req) => {
                            if connection.handle_shutdown(&req)? {
                                return Ok(());
                            }
                            self.on_request(req);
                        }
                        Message::Notification(not) => self.on_notification(not),
                        Message::Response(_) => {}
                    }
                }
                recv(self.events_rx) -> event => {
                    if let Ok(event) = event {
                        self.on_event(event);
                    }
                }
            }
        }
    }

    fn send(&self, msg: Message) {
        if self.sender.send(msg).is_err() {
            log::error!("the client connection is closed");
        }
    }

    fn notify<N: lsp_types::notification::Notification>(&self, params: N::Params) {
        self.send(Notification::new(N::METHOD.to_owned(), params).into());
    }

    // === Requests ===

    fn on_request(&mut self, req: Request) {
        use lsp_types::request::*;
        match req.method.as_str() {
            Completion::METHOD => self.respond(req, handlers::completion),
            HoverRequest::METHOD => self.respond(req, handlers::hover),
            GotoDefinition::METHOD => self.respond(req, handlers::definition),
            GotoDeclaration::METHOD => self.respond(req, handlers::definition),
            SignatureHelpRequest::METHOD => self.respond(req, handlers::signature_help),
            InlayHintRequest::METHOD => self.respond(req, handlers::inlay_hints),
            SemanticTokensFullRequest::METHOD => self.respond(req, handlers::semantic_tokens_full),
            SemanticTokensRangeRequest::METHOD => {
                self.respond(req, handlers::semantic_tokens_range)
            }
            CodeActionRequest::METHOD => self.respond(req, handlers::code_actions),
            CodeLensRequest::METHOD => self.respond(req, handlers::code_lenses),
            DocumentLinkRequest::METHOD => self.respond(req, handlers::document_links),
            Formatting::METHOD => self.respond(req, handlers::formatting),
            References::METHOD => self.respond(req, handlers::references),
            DocumentHighlightRequest::METHOD => self.respond(req, handlers::highlights),
            DocumentSymbolRequest::METHOD => self.respond(req, handlers::document_symbols),
            WorkspaceSymbolRequest::METHOD => self.respond(req, handlers::workspace_symbols),
            PrepareRenameRequest::METHOD => self.respond(req, handlers::prepare_rename),
            Rename::METHOD => self.respond(req, handlers::rename),
            FoldingRangeRequest::METHOD => self.respond(req, handlers::folding_ranges),
            "xmake-analyzer/reload" => {
                self.start_jobs();
                self.send(Response::new_ok(req.id, ()).into());
            }
            _ => {
                let message = format!("unknown request {}", req.method);
                self.send(
                    Response::new_err(req.id, ErrorCode::MethodNotFound as i32, message).into(),
                );
            }
        }
    }

    /// Answers `req` with `handler`, turning bad parameters and panics into
    /// error responses.
    fn respond<P, R>(&self, req: Request, handler: fn(&Server, P) -> R)
    where
        P: DeserializeOwned,
        R: Serialize,
    {
        let response = match serde_json::from_value::<P>(req.params) {
            Ok(params) => match catch_unwind(AssertUnwindSafe(|| handler(self, params))) {
                Ok(result) => Response::new_ok(req.id, result),
                Err(_) => {
                    log::error!("panic while handling {}", req.method);
                    let message = format!("internal error in {}", req.method);
                    Response::new_err(req.id, ErrorCode::InternalError as i32, message)
                }
            },
            Err(err) => Response::new_err(req.id, ErrorCode::InvalidParams as i32, err.to_string()),
        };
        self.send(response.into());
    }

    // === Notifications ===

    fn on_notification(&mut self, not: Notification) {
        use lsp_types::notification::*;
        let result = match not.method.as_str() {
            DidOpenTextDocument::METHOD => parse(not).map(|p| self.on_open(p)),
            DidChangeTextDocument::METHOD => parse(not).map(|p| self.on_change(p)),
            DidCloseTextDocument::METHOD => parse(not).map(|p| self.on_close(p)),
            DidChangeConfiguration::METHOD => parse(not).map(|p| self.on_configuration(p)),
            DidChangeWatchedFiles::METHOD => parse(not).map(|p| self.on_watched_files(p)),
            _ => Ok(()),
        };
        if let Err(err) = result {
            log::warn!("invalid notification: {err}");
        }
    }

    fn on_initialized(&mut self) {
        if self.watch_files {
            self.register_file_watcher();
        }
        self.start_jobs();
    }

    fn register_file_watcher(&mut self) {
        let options = lsp_types::DidChangeWatchedFilesRegistrationOptions {
            watchers: vec![lsp_types::FileSystemWatcher {
                glob_pattern: lsp_types::GlobPattern::String("**/*.lua".to_owned()),
                kind: None,
            }],
        };
        let params = lsp_types::RegistrationParams {
            registrations: vec![lsp_types::Registration {
                id: "xmake-analyzer-watcher".to_owned(),
                method: "workspace/didChangeWatchedFiles".to_owned(),
                register_options: serde_json::to_value(options).ok(),
            }],
        };
        self.next_request_id += 1;
        let id = lsp_server::RequestId::from(self.next_request_id);
        self.send(Request::new(id, "client/registerCapability".to_owned(), params).into());
    }

    /// Scans the workspace and probes xmake in the background.
    fn start_jobs(&mut self) {
        let roots = self.roots.clone();
        let tx = self.events_tx.clone();
        spawn("xmake-analyzer-scan", move || {
            let _ = tx.send(Event::Scanned(scan::scan(&roots)));
        });
        self.repositories = None;
        self.load_packages();
        if self.config.query_xmake {
            let xmake = self.config.xmake_path.clone();
            let tx = self.events_tx.clone();
            spawn("xmake-analyzer-probe", move || {
                let _ = tx.send(Event::Probed(probe::probe(&xmake).map(Box::new)));
            });
        }
    }

    fn on_open(&mut self, params: DidOpenTextDocumentParams) {
        let doc = params.text_document;
        let Some(path) = conv::uri_to_path(&doc.uri) else {
            return;
        };
        self.project.set_file(path.clone(), Arc::from(doc.text));
        self.open.insert(path, OpenDoc { uri: doc.uri, version: doc.version });
        self.update_includes();
        self.publish_all();
    }

    fn on_change(&mut self, params: DidChangeTextDocumentParams) {
        let Some(path) = conv::uri_to_path(&params.text_document.uri) else {
            return;
        };
        let Some(file) = self.project.file(&path) else {
            return;
        };
        let mut text = String::from(&*file.text);
        for change in params.content_changes {
            match change.range {
                Some(range) => {
                    // Positions refer to the text after the previous change.
                    let index = LineIndex::new(&text);
                    let at = |p: lsp_types::Position| -> usize {
                        index.offset(&text, LineCol { line: p.line, col: p.character }).into()
                    };
                    let (start, end) = (at(range.start), at(range.end));
                    text.replace_range(start..end.max(start), &change.text);
                }
                None => text = change.text,
            }
        }
        let old_includes: Vec<String> =
            Project::include_values(file).into_iter().map(str::to_owned).collect();
        let old_repositories = file.outline.repositories.clone();
        let old_imports: Vec<String> =
            Project::import_values(file).into_iter().map(str::to_owned).collect();
        let file = self.project.set_file(path.clone(), Arc::from(text));
        let doc = params.text_document;
        self.open.insert(path, OpenDoc { uri: doc.uri, version: doc.version });
        // Following includes reads the disk: only when they were edited.
        if Project::include_values(&file) != old_includes
            || Project::import_values(&file) != old_imports
        {
            self.update_includes();
        } else if file.outline.repositories != old_repositories {
            self.load_packages();
        }
        self.publish_all();
    }

    fn on_close(&mut self, params: DidCloseTextDocumentParams) {
        let Some(path) = conv::uri_to_path(&params.text_document.uri) else {
            return;
        };
        let uri = self.open.remove(&path).map_or_else(|| conv::path_to_uri(&path), |d| d.uri);
        // Keep the project files known from the disk; forget the others.
        let on_disk =
            self.is_project_file(&path).then(|| std::fs::read_to_string(&path).ok()).flatten();
        match on_disk {
            Some(text) => {
                self.project.set_file(path.clone(), Arc::from(text));
            }
            None => self.project.remove_file(&path),
        }
        self.notify::<lsp_types::notification::PublishDiagnostics>(PublishDiagnosticsParams {
            uri,
            diagnostics: Vec::new(),
            version: None,
        });
        self.publish_all();
    }

    fn on_configuration(&mut self, params: DidChangeConfigurationParams) {
        let config = Config::from_json(Some(&params.settings));
        if config == self.config {
            return;
        }
        let reprobe = config.xmake_path != self.config.xmake_path
            || config.query_xmake != self.config.query_xmake;
        self.config = config;
        if !self.config.query_xmake {
            self.project.db = ApiDb::builtin();
            self.luajit = None;
        }
        self.update_dialect();
        if reprobe {
            self.start_jobs();
        }
        self.publish_all();
    }

    fn on_watched_files(&mut self, params: DidChangeWatchedFilesParams) {
        for change in params.changes {
            let Some(path) = conv::uri_to_path(&change.uri) else {
                continue;
            };
            if self.open.contains_key(&path) {
                continue;
            }
            if change.typ == FileChangeType::DELETED {
                self.project.remove_file(&path);
                continue;
            }
            let known = self.project.file(&path).is_some();
            if known || self.is_project_file(&path) {
                match std::fs::read_to_string(&path) {
                    Ok(text) => {
                        self.project.set_file(path, Arc::from(text));
                    }
                    Err(err) => log::warn!("cannot read {}: {err}", path.display()),
                }
            }
        }
        self.update_includes();
        self.publish_all();
    }

    // === Background jobs ===

    fn on_event(&mut self, event: Event) {
        match event {
            Event::Probed(Ok(probe)) => {
                log::info!(
                    "using the API lists of xmake {} ({})",
                    probe.version,
                    if probe.luajit { "LuaJIT" } else { "Lua" }
                );
                let probe = *probe;
                self.project.db = probe.db;
                self.project.xmake_version = Some(probe.version);
                self.luajit = Some(probe.luajit);
                self.update_dialect();
            }
            Event::Probed(Err(err)) => {
                log::warn!("{err}; using the builtin API lists");
                self.show_message(
                    lsp_types::MessageType::WARNING,
                    format!("xmake-analyzer: {err}. Using the builtin API lists."),
                );
            }
            Event::Packages(index, repositories) => {
                // A load for repositories since changed: another one comes.
                if self.repositories.as_ref() != Some(&repositories) {
                    return;
                }
                log::info!("found {} packages in the repositories", index.entries().len());
                self.project.packages = index;
            }
            Event::Scanned(files) => {
                log::info!("found {} xmake.lua files", files.len());
                for (path, text) in files {
                    if !self.open.contains_key(&path) {
                        self.project.set_file(path, Arc::from(text));
                    }
                }
                self.update_includes();
            }
        }
        self.publish_all();
    }

    fn show_message(&self, typ: lsp_types::MessageType, message: String) {
        self.notify::<lsp_types::notification::ShowMessage>(lsp_types::ShowMessageParams {
            typ,
            message,
        });
    }

    fn update_dialect(&mut self) {
        let dialect = self.dialect();
        if dialect != self.project.dialect {
            log::info!("checking scripts as {}", dialect.lua);
            self.project.dialect = dialect;
            self.project.reanalyze();
        }
    }

    /// Follows the `includes()` of the project, and tells the client which
    /// `.lua` files are description files, so that it gives them the `xmake`
    /// language instead of `lua` (no Lua language server on them).
    /// Indexes the package repositories again when the local ones
    /// (`add_repositories`) changed.
    fn load_packages(&mut self) {
        let repositories = self.project.local_repositories();
        if self.repositories.as_ref() == Some(&repositories) {
            return;
        }
        self.repositories = Some(repositories.clone());
        let tx = self.events_tx.clone();
        spawn("xmake-analyzer-packages", move || {
            let globaldir = xmake_ide::default_globaldir();
            let index = xmake_ide::PackageIndex::load(globaldir.as_deref(), &repositories);
            let _ = tx.send(Event::Packages(Arc::new(index), repositories));
        });
    }

    /// Follows the includes, and tells the client the xmake files that are
    /// not named `xmake.lua`: the included files and the imported modules.
    fn update_includes(&mut self) {
        self.project.update_includes();
        self.load_packages();
        let modules = self.project.imported_modules();
        let mut uris: Vec<String> = self
            .project
            .included_files()
            .chain(modules.iter().map(PathBuf::as_path))
            .map(|p| conv::path_to_uri(p).as_str().to_owned())
            .collect();
        uris.sort();
        uris.dedup();
        if self.xmake_files.as_ref() == Some(&uris) {
            return;
        }
        self.xmake_files = Some(uris.clone());
        log::info!("{} files included or imported by xmake.lua files", uris.len());
        self.send(
            Notification::new("xmake-analyzer/includedFiles".to_owned(), IncludedFiles { uris })
                .into(),
        );
    }

    /// Whether `path` is a description file of the project: an `xmake.lua`
    /// under a workspace root, or a file it includes.
    fn is_project_file(&self, path: &Path) -> bool {
        let is_description = path.file_name().is_some_and(|n| n.eq_ignore_ascii_case("xmake.lua"));
        let in_roots = is_description && self.roots.iter().any(|r| path.starts_with(r));
        in_roots || self.project.included_files().any(|p| p == path)
    }

    // === Diagnostics ===

    fn publish_all(&self) {
        for (path, doc) in &self.open {
            self.publish(path, doc);
        }
    }

    fn publish(&self, path: &Path, doc: &OpenDoc) {
        let Some(file) = self.project.file(path) else {
            return;
        };
        let diagnostics = if self.config.diagnostics {
            xmake_ide::diagnostics(&self.project, file, &self.config.lints)
                .iter()
                .map(|d| handlers::diagnostic(file, d))
                .collect()
        } else {
            Vec::new()
        };
        self.notify::<lsp_types::notification::PublishDiagnostics>(PublishDiagnosticsParams {
            uri: doc.uri.clone(),
            diagnostics,
            version: Some(doc.version),
        });
    }

    /// The analysis of the document `uri`.
    pub(crate) fn file(
        &self,
        uri: &lsp_types::Uri,
    ) -> Option<(PathBuf, &Arc<xmake_ide::AnalyzedFile>)> {
        let path = conv::uri_to_path(uri)?;
        let file = self.project.file(&path)?;
        Some((path, file))
    }
}

fn parse<P: DeserializeOwned>(not: Notification) -> Result<P, serde_json::Error> {
    serde_json::from_value(not.params)
}

fn spawn(name: &str, job: impl FnOnce() + Send + 'static) {
    let spawned = std::thread::Builder::new().name(name.to_owned()).spawn(job);
    if let Err(err) = spawned {
        log::error!("cannot start {name}: {err}");
    }
}
