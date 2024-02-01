mod error;
mod slither;
mod types;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

use crate::error::SlitherError;
use crate::slither::*;
use tokio::process::{Child, Command};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct SlitherDiag {
    diagnostics: Vec<Diagnostic>,
    uri: Url,
}

impl SlitherDiag {
    fn new(uri: Url, diagnostics: Vec<Diagnostic>) -> Self {
        Self { uri, diagnostics }
    }
}

#[derive(Debug)]
struct SlitherData {
    slither_processes: Vec<CancellationToken>,
    receiver: Option<Receiver<SlitherDiag>>,
    sender: Sender<SlitherDiag>,
    libs_paths: Vec<String>,
    src_paths: Vec<String>,
    tests_paths: Vec<String>,
}

#[derive(Debug)]
struct Backend {
    client: Client,
    data: Mutex<SlitherData>,
    join_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl SlitherData {
    fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel::<SlitherDiag>(100);
        Self {
            libs_paths: vec![],
            src_paths: vec![],
            tests_paths: vec![],
            slither_processes: vec![],
            receiver: Some(receiver),
            sender,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if !is_slither_installed() {
            self.client
                .show_message(
                    MessageType::ERROR,
                    "Slither is not installed! Please install it and restart the extension",
                )
                .await;
            self.client
                .log_message(MessageType::ERROR, "Slither is not installed!")
                .await;
            return Err(tower_lsp::jsonrpc::Error::internal_error());
        }
        if !is_solc_installed() {
            self.client
                .show_message(
                    MessageType::ERROR,
                    "Solc is not installed! Please install it and restart the extension",
                )
                .await;
            self.client
                .log_message(MessageType::ERROR, "Solc is not installed!")
                .await;
            return Err(tower_lsp::jsonrpc::Error::internal_error());
        }

        self.client
            .log_message(MessageType::INFO, "diag recv initializing ...")
            .await;
        let mut receiver = self.data.lock().await.receiver.take().unwrap();
        let client = self.client.clone();

        self.join_handle
            .lock()
            .await
            .replace(tokio::spawn(async move {
                while let Some(diagnostics) = receiver.recv().await {
                    client
                        .publish_diagnostics(diagnostics.uri, diagnostics.diagnostics, None)
                        .await;
                }
            }));
        self.client
            .log_message(MessageType::INFO, "diag recv initialized!")
            .await;

        let folders = params.workspace_folders;
        if let Some(folder) = folders {
            eprintln!("Initializing filters ...");
            match self.initialize_filters(folder).await {
                Ok(_) => {
                    eprintln!("Filters initialized!");
                }
                Err(e) => {
                    eprintln!("Error while initializing filters: {:?}", e);
                }
            }
        } else {
            eprintln!("No workspace folders found!");
        }

        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "osmium-slither initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        let state = self.data.lock().await;
        for process in state.slither_processes.iter() {
            process.cancel();
        }
        self.join_handle.lock().await.take().unwrap().abort();
        Ok(())
    }

    async fn did_save(&self, file: DidSaveTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Saved file '{}' for analyzing.",
                    file.text_document.uri.path()
                ),
            )
            .await;
        if self.is_in_libs(file.text_document.uri.path()).await
          //  || self.is_in_tests(file.text_document.uri.path()).await
          //  || !self.is_in_src(file.text_document.uri.path()).await
        {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "File '{}' is not a source solidity code file, skipping analysis.",
                        file.text_document.uri.path()
                    ),
                )
                .await;
            return;
        }
        self.check_slither_result(file.text_document.uri).await
    }
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            data: Mutex::new(SlitherData::new()),
            join_handle: Arc::new(Mutex::new(None)),
        }
    }

    async fn is_in_libs(&self, path: &str) -> bool {
        let state = self.data.lock().await;
        for lib in state.libs_paths.iter() {
            let fsrc = format!("/{}/", lib.replace("\"", ""));
            eprintln!("Check path: '{}' contains lib: '{}'", path, fsrc);
            if path.contains(&fsrc) {
                return true;
            }
        }
        false
    }

    async fn is_in_src(&self, path: &str) -> bool {
        let state = self.data.lock().await;
        for src in state.src_paths.iter() {
            let fsrc = format!("/{}/", src.replace("\"", ""));
            eprintln!("Check path: '{}' contains src: '{}'", path, fsrc);
            if path.contains(&fsrc) {
                return true;
            }
        }
        false
    }

    async fn is_in_tests(&self, path: &str) -> bool {
        if path.ends_with(".t.sol") {
            return true;
        }
        let state = self.data.lock().await;
        for test in state.tests_paths.iter() {
            let fsrc = format!("/{}/", test.replace("\"", ""));
            eprintln!("Check path: '{}' contains test: '{}'", path, fsrc);
            if path.contains(&fsrc) {
                return true;
            }
        }
        false
    }

    async fn initialize_filters(&self, workspaces: Vec<WorkspaceFolder>) -> Result<()> {
        let mut state = self.data.lock().await;
        //register all work directories folder aliases using foundry.toml for each workspace folder
        for folder in workspaces {
            let folderpath = normalize_slither_path(folder.uri.path());
            let foundry_path = find_foundry_toml_config(&folderpath);
            match foundry_path {
                Ok(path) => {
                    let foundry = std::fs::read_to_string(path.clone());
                    match foundry {
                        Ok(foundry) => {
                            let foundry: toml::Value = foundry.parse().unwrap();
                            let libs = foundry["profile"]["default"]["libs"].as_array();
                            match libs {
                                Some(libs) => {
                                    for lib in libs {
                                        state.libs_paths.push(lib.to_string());
                                    }
                                }
                                None => {
                                    state
                                        .libs_paths
                                        .push(foundry["profile"]["default"]["libs"].to_string());
                                }
                            }
                            let src = foundry["profile"]["default"]["src"].as_array();
                            match src {
                                Some(src) => {
                                    for src in src {
                                        state.src_paths.push(src.to_string());
                                    }
                                }
                                None => {
                                    state
                                        .src_paths
                                        .push(foundry["profile"]["default"]["src"].to_string());
                                }
                            }
                            let tests = foundry["profile"]["default"]["test"].as_array();
                            match tests {
                                Some(tests) => {
                                    for test in tests {
                                        state.tests_paths.push(test.to_string());
                                    }
                                }
                                None => {
                                    state
                                        .tests_paths
                                        .push(foundry["profile"]["default"]["test"].to_string());
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "Error while reading foundry.toml file: {:?}, path: {}",
                                e, path
                            );
                        }
                    }
                }

                Err(_) => {}
            }
        }
        Ok(())
    }

    async fn check_slither_result(&self, uri: Url) {
        let token = CancellationToken::new();
        let clone = token.clone();
        self.data.lock().await.slither_processes.push(token);
        let sender_handle = self.data.lock().await.sender.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = clone.cancelled() => {
                    eprintln!("SLITHER CANCELLED");
                    return;
                }
                output = parse_slither_out(uri.path()) => {
                    match output {
                        Ok(res) => {
                            let _ = sender_handle.send(SlitherDiag::new(uri, res)).await;
                        },
                        Err(SlitherError::ParsingFailed(e)) => {
                            client
                                .log_message(
                                    MessageType::ERROR,
                                    format!(
                                        "File '{}' did generate an error while parsing the output: {:?}",
                                        uri.path(),
                                        e
                                    ),
                                )
                                .await;
                            client.publish_diagnostics(uri, vec![], None).await;
                        }
                        Err(e) => {
                            client
                                .log_message(
                                    MessageType::ERROR,
                                    format!("File '{}' did generate an error: {:?}", uri.path(), e),
                                )
                                .await;
                        }
                    }
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
