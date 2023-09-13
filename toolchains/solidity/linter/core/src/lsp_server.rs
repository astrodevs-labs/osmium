use std::sync::{Arc};
use tokio::sync::Mutex;

use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::types::LintDiag;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::Args;

struct Backend {
    client: Client,
    config_file_path: String,
    working_dir: String,
    linter: Arc<Mutex<Option<SolidLinter>>>
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Server initialized!")
            .await;

        let file_res = std::fs::read_to_string(&self.config_file_path);

        if let Ok(file) = file_res {
            let mut linter = self.linter.lock().await;
            linter.replace(SolidLinter::new(&file));
        } else {
            let mut linter = self.linter.lock().await;
            linter.replace(SolidLinter::new_fileless());
        }

        self.client
            .log_message(MessageType::INFO, "Linter initialized!")
            .await;

    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "Server shutdown!")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;

        let mut linter = self.linter.lock().await;
        let filepath = filepath_from_uri(&params.text_document.uri);
        let linter = linter.as_mut().unwrap();
        let diags_res = linter.parse_content(filepath, &params.text_document.text);

        if let Ok(diags) = diags_res {
            let diags = diags.iter().map(|d| diagnostic_from_lintdiag(d.clone())).collect();
            self.client.log_message(MessageType::INFO, "diags: ").await;
            self.client.publish_diagnostics(params.text_document.uri.clone(), diags, None).await;
            //self.client.publish_diagnostics(params.text_document.uri, diags, None).await;
        } else if let Err(e) = diags_res {
            self.client.log_message(MessageType::ERROR, e.to_string()).await;
        }
        
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;

            let mut linter = self.linter.lock().await;
            let filepath = filepath_from_uri(&params.text_document.uri);
            let linter = linter.as_mut().unwrap();
            let diags_res = linter.parse_content(filepath, &params.content_changes[0].text);
    
            if let Ok(diags) = diags_res {
                let diags = diags.iter().map(|d| diagnostic_from_lintdiag(d.clone())).collect();
                self.client.log_message(MessageType::INFO, "diags: ").await;
                self.client.publish_diagnostics(params.text_document.uri.clone(), diags, None).await;
                //self.client.publish_diagnostics(params.text_document.uri, diags, None).await;
            } else if let Err(e) = diags_res {
                self.client.log_message(MessageType::ERROR, e.to_string()).await;
            }
    }
}

pub fn filepath_from_uri(uri: &Url) -> String {
    let path = uri.path();
//    let path = path.strip_prefix("/").unwrap_or(path);
    path.to_string()
}

fn diagnostic_from_lintdiag(diag: LintDiag) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position {
                line: diag.range.start.line as u32,
                character: diag.range.start.character as u32,
            },
            end: Position {
                line: diag.range.end.line as u32,
                character: diag.range.end.character as u32,
            },
        },
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: Some("solidity-linter".to_string()),
        message: diag.message,
        related_information: None,
        tags: None,
        data: None,
    }
}

#[tokio::main]
pub async fn run_server(args: Args) {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { 
        client,
        config_file_path: args.rules_file,
        working_dir: args.project_path[0].clone(),
        linter: Arc::new(Mutex::new(None))
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}