use ast_extractor::{Item, Spanned};
use ast_extractor::Visibility::{Internal, Private};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "private-vars-leading-underscore";
const MESSAGE: &str = "Private and internal names must start with a single underscore";

const DEFAULT_STRICT: bool = false;

pub struct PrivateVarsLeadingUnderscore {
    data: RuleEntry,
    config: serde_json::Value,
}

impl PrivateVarsLeadingUnderscore {
    fn create_diag(
        &self,
        location: (ast_extractor::LineColumn, ast_extractor::LineColumn),
        file: &SolidFile,
    ) -> LintDiag {
        LintDiag {
            id: RULE_ID.to_string(),
            range: Range {
                start: Position {
                    line: location.0.line,
                    character: location.0.column,
                },
                end: Position {
                    line: location.1.line,
                    character: location.1.column,
                },
            },
            message: MESSAGE.to_string(),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for PrivateVarsLeadingUnderscore {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            let functions = ast_extractor::retriever::retrieve_functions_nodes(&contract);

            for function in functions {
                let is_private = match function.attributes.visibility() {
                    Some(val) => match val {
                        Private(_) => true,
                        Internal(_) => true,
                        _ => false,
                    },
                    None => true,
                };

                if let Some(name) = function.name {
                    let leading_underscore = name.as_string().starts_with('_');

                    if !leading_underscore && is_private {
                        let span = name.span();
                        res.push(self.create_diag((span.start(), span.end()), file));
                    }
                }
            }

            for node_var in contract.body.iter() {
                if let Item::Variable(var) = node_var {
                    let is_private = match var.attributes.visibility() {
                        Some(val) => match val {
                            Private(_) => true,
                            Internal(_) => true,
                            _ => false,
                        },
                        None => true,
                    };

                    let leading_underscore = var.name.as_string().starts_with('_');

                    if !leading_underscore && is_private {
                        let span = var.name.span();
                        res.push(self.create_diag((span.start(), span.end()), file));
                    }
                }
            }
        }
        res
    }
}

impl PrivateVarsLeadingUnderscore {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut strict = DEFAULT_STRICT;

        if !data.data.is_empty() {
            strict = match data.data[0].as_bool() {
                Some(val) => val as bool,
                None => DEFAULT_STRICT,
            };
        }
        let rule = PrivateVarsLeadingUnderscore {
            data,
            config: serde_json::json!({
                "strict": strict,
            }),
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![
                serde_json::json!({
                    "strict": DEFAULT_STRICT,
                }),
            ],
        }
    }
}
