use osmium_libs_lsp_server_wrapper::{
    lsp_types::*, Client, LanguageServer, LspStdioServer, RequestId, Result,
};
use solidhunter_lib::{linter::SolidLinter, types::LintDiag};
use std::{cell::RefCell, rc::Rc};
mod utils;
use utils::get_closest_config_filepath;
mod get_content;
use get_content::{ContentRequest, ContentRequestParams, ContentResponse};

struct Backend {
    connection: Rc<RefCell<Client>>,
    linter: RefCell<Option<SolidLinter>>,
}

impl LanguageServer for Backend {
    fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let connection = self.connection.borrow_mut();
        connection.log_message(MessageType::INFO, "Server initializing!");
        if let Ok(closest_config_path) = get_closest_config_filepath(&connection, params.clone()) {
            if let Some(path) = closest_config_path {
                connection.log_message(
                    MessageType::INFO,
                    &format!("Initializing linter with workspace path: {:?}", path),
                );
                let mut linter = SolidLinter::new();

                let res = linter.initialize_rules(&path);
                if res.is_ok() {
                    self.linter.replace(Some(linter));
                } else {
                    connection.log_message(
                        MessageType::ERROR,
                        "Failed to initialize linter with workspace path, using fileless linter",
                    );
                    let linter = SolidLinter::new_fileless();
                    self.linter.replace(Some(linter));
                }
            } else {
                connection.log_message(
                    MessageType::INFO,
                    "Initializing linter without workspace path1",
                );
                let linter = SolidLinter::new_fileless();
                self.linter.replace(Some(linter));
            }
        } else {
            connection.log_message(
                MessageType::INFO,
                "Initializing linter without workspace path2",
            );
            let linter = SolidLinter::new_fileless();
            self.linter.replace(Some(linter));
        }
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
        self.connection
            .borrow_mut()
            .log_message(MessageType::INFO, "Linter initialized!");
    }

    fn shutdown(&self) -> Result<()> {
        self.connection
            .borrow_mut()
            .log_message(MessageType::INFO, "Server shutdown!");
        Ok(())
    }

    fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.connection.borrow_mut().log_message(
            MessageType::INFO,
            format!("file opened!: {:}", params.text_document.uri),
        );

        self.lint(params.text_document.uri, params.text_document.text);
    }

    fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.connection.borrow_mut().log_message(
            MessageType::INFO,
            format!("file changed!: {:}", params.text_document.uri),
        );

        self.lint(
            params.text_document.uri,
            params.content_changes[0].text.clone(),
        );
    }

    fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        self.connection
            .borrow_mut()
            .log_message(MessageType::INFO, "configuration file changed!");

        if params.changes[0].typ == FileChangeType::DELETED {
            return;
        }

        let params = ContentRequestParams {
            uri: params.changes[0].uri.path().to_string().clone(),
        };

        let res = self
            .connection
            .borrow_mut()
            .send_request::<ContentRequest>(params.clone());

        if res.is_err() {
            self.connection.borrow_mut().log_message(
                MessageType::ERROR,
                "Failed to send request to get configuration file content!",
            );
            return;
        }

        self.connection.borrow_mut().log_message(
            MessageType::INFO,
            format!(
                "Sent request to get file content for config file : {:}",
                params.uri
            ),
        );
    }

    fn on_response(&self, _: RequestId, result: Option<serde_json::Value>) {
        self.connection
            .borrow_mut()
            .log_message(MessageType::INFO, "Got response!");

        if result.is_none() {
            self.connection
                .borrow_mut()
                .log_message(MessageType::ERROR, "Get content response is empty!");
            return;
        }

        eprintln!("result: {:#?}", result.clone().unwrap());

        let res: serde_json::Result<ContentResponse> =
            serde_json::from_value::<ContentResponse>(result.unwrap());
        if res.is_err() {
            self.connection
                .borrow_mut()
                .log_message(MessageType::ERROR, "Failed to parse response!");
            return;
        }
        let response: ContentResponse = res.unwrap();

        let mut linter = SolidLinter::new();
        let res = linter.initialize_rules_content(&response.content);
        if res.is_ok() {
            self.connection
                .borrow_mut()
                .log_message(MessageType::INFO, "configuration file loaded!");
            self.linter.replace(Some(linter));
        } else {
            self.connection
                .borrow_mut()
                .log_message(MessageType::ERROR, "configuration file failed to load!");
        }
    }
}

impl Backend {
    pub fn new(connection: Rc<RefCell<Client>>) -> Self {
        Self {
            connection,
            linter: RefCell::new(None),
        }
    }

    pub fn lint(&self, uri: Url, text: String) {
        let filepath = filepath_from_uri(&uri);
        let mut linter = self.linter.borrow_mut();
        let linter = match linter.as_mut() {
            Some(l) => l,
            None => {
                eprintln!("Linter cannot be ran due to previous errors");
                return;
            }
        };
        let diags_res = linter.parse_content(&filepath, &text);

        if let Ok(diags) = diags_res {
            let diags = diags
                .diags
                .iter()
                .map(|d| diagnostic_from_lintdiag(d.clone()))
                .collect();
            eprintln!("diags: {:#?}", diags);
            self.connection
                .borrow_mut()
                .publish_diagnostics(uri.clone(), diags, None);
        } else if let Err(e) = diags_res {
            self.connection
                .borrow_mut()
                .log_message(MessageType::ERROR, e.to_string());
        }
    }
}

pub fn filepath_from_uri(uri: &Url) -> String {
    let path = uri.path();
    path.to_string()
}

fn diagnostic_from_lintdiag(diag: LintDiag) -> Diagnostic {
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
        source: Some("osmium-solidity-linter".to_string()),
        message: diag.message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn main() -> std::result::Result<(), usize> {
    let server = LspStdioServer::new();
    LspStdioServer::serve(server, Backend::new).map_err(|err| {
        eprintln!("Error: {:?}", err);
        1
    })
}
