mod error;
mod slither;
mod types;

use crate::error::SlitherError;
use crate::slither::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        if !is_slither_installed() {
            self.client
                .log_message(MessageType::ERROR, "Slither is not installed!")
                .await;
            return Err(tower_lsp::jsonrpc::Error::internal_error());
        }
        if !is_solc_installed() {
            self.client
                .log_message(MessageType::ERROR, "Solc is not installed!")
                .await;
            return Err(tower_lsp::jsonrpc::Error::internal_error());
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
        Ok(())
    }

    async fn did_open(&self, file: DidOpenTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Opened file '{}' for analyzing.",
                    file.text_document.uri.path()
                ),
            )
            .await;
        self.check_slither_result(file.text_document.uri).await
    }

    async fn did_change(&self, file: DidChangeTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Changed file '{}' for analyzing.",
                    file.text_document.uri.path()
                ),
            )
            .await;
        self.check_slither_result(file.text_document.uri).await
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
        self.check_slither_result(file.text_document.uri).await
    }
}

impl Backend {
    async fn check_slither_result(&self, uri: Url) {
        let res = exec_slither(uri.path());
        match res {
            Ok(res) => {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "File '{}' did generate {} security diagnostics.",
                            uri.path(),
                            res.len()
                        ),
                    )
                    .await;
                self.client.publish_diagnostics(uri, res, None).await;
            }
            Err(SlitherError::ParsingFailed(e)) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!(
                            "File '{}' did generate an error while parsing the output: {:?}",
                            uri.path(),
                            e
                        ),
                    )
                    .await;
                self.client.publish_diagnostics(uri, vec![], None).await;
            }
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("File '{}' did generate an error: {:?}", uri.path(), e),
                    )
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
