use std::{cell::RefCell, rc::Rc};

use lsp_server_wrapper::{ LanguageServer, Client, lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, DidOpenTextDocumentParams, DidChangeTextDocumentParams, Diagnostic, DiagnosticSeverity, Range, Position, Url}, Result, LspStdioServer };
use solidhunter_lib::{linter::SolidLinter, types::LintDiag};

struct Backend {
    client: Rc<RefCell<Client>>,
    config_file_path: String,
    linter: RefCell<Option<SolidLinter>>
}

impl LanguageServer for Backend {
    fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        eprintln!("starting example main loop");
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

    fn initialized(&self, _: InitializedParams) {
        eprintln!("Initializing server with config file: {:?}", self.config_file_path);
        self.client.as_ref().borrow()
            .log_message(MessageType::INFO, "Server initialized!");

        let file_res = std::fs::read_to_string(&self.config_file_path);

        if let Ok(file) = file_res {
            self.linter.borrow_mut().replace(SolidLinter::new(&file));
        } else {
            self.linter.borrow_mut().replace(SolidLinter::new_fileless());
        }

        self.client.as_ref().borrow()
            .log_message(MessageType::INFO, "Linter initialized!");

    }

    fn shutdown(&self) -> Result<()> {
        self.client.as_ref().borrow()
            .log_message(MessageType::INFO, "Server shutdown!");
        Ok(())
    }

    fn did_open(&self, params: DidOpenTextDocumentParams) {
        eprintln!("file opened!");
        self.client.as_ref().borrow()
            .log_message(MessageType::INFO, "file opened!");

        let filepath = filepath_from_uri(&params.text_document.uri);
        let mut linter = self.linter.borrow_mut();
        let linter = linter.as_mut().unwrap();
        let diags_res = linter.parse_content(filepath, &params.text_document.text);

        if let Ok(diags) = diags_res {
            let diags = diags.iter().map(|d| diagnostic_from_lintdiag(d.clone())).collect();
            eprintln!("diags: {:?}", diags);
            self.client.as_ref().borrow().log_message(MessageType::INFO, "diags: ");
            self.client.as_ref().borrow().publish_diagnostics(params.text_document.uri.clone(), diags, None);
            //self.client.publish_diagnostics(params.text_document.uri, diags, None).await;
        } else if let Err(e) = diags_res {
            self.client.as_ref().borrow().log_message(MessageType::ERROR, e.to_string());
        }
        
    }

    fn did_change(&self, params: DidChangeTextDocumentParams) {
        eprintln!("file changed!");
        self.client.as_ref().borrow()
            .log_message(MessageType::INFO, "file changed!");

        let filepath = filepath_from_uri(&params.text_document.uri);
        let mut linter = self.linter.borrow_mut();
        let linter = linter.as_mut().unwrap();
        let diags_res = linter.parse_content(filepath, &params.content_changes[0].text);

        if let Ok(diags) = diags_res {
            let diags = diags.iter().map(|d| diagnostic_from_lintdiag(d.clone())).collect();
            eprintln!("diags: {:?}", diags);
            self.client.as_ref().borrow().publish_diagnostics(params.text_document.uri.clone(), diags, None);
            //self.client.publish_diagnostics(params.text_document.uri, diags, None).await;
        } else if let Err(e) = diags_res {
            self.client.as_ref().borrow().log_message(MessageType::ERROR, e.to_string());
        }
    }
}

pub fn filepath_from_uri(uri: &Url) -> String {
    let path = uri.path();
//    let path = path.strip_prefix("/").unwrap_or(path);
    path.to_string()
}

fn diagnostic_from_lintdiag(diag: LintDiag) -> Diagnostic {
    eprintln!("diag: {:?}", diag);
    Diagnostic {
        range: Range {
            start: Position {
                line: diag.range.start.line as u32 - 1,
                character: diag.range.start.character as u32,
            },
            end: Position {
                line: diag.range.end.line as u32 - 1,
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

pub fn run_server(config_file_path: String) {
    eprintln!("starting LSP server");
    let server = LspStdioServer::new();
    let _ = LspStdioServer::serve(server, |client| Backend { 
        client,
        config_file_path,
        linter: RefCell::new(None)
    });
}