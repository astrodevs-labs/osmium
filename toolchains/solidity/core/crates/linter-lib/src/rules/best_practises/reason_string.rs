use osmium_libs_solidity_ast_extractor::*;

use crate::linter::SolidFile;
use crate::rules::types::{RuleEntry, RuleType};
use crate::types::{LintDiag, Position, Range, Severity};

// global
pub const RULE_ID: &str = "reason-string";

// Specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_LENGTH: usize = 32;

pub struct ReasonString {
    max_length: usize,
    data: RuleEntry,
}

impl ReasonString {
    fn create_diag(
        &self,
        file: &SolidFile,
        location: (LineColumn, LineColumn),
        message: String,
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
            message,
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for ReasonString {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for contract in retriever::retrieve_contract_nodes(&file.data) {
            for stmt in retriever::retrieve_stmts_nodes(&contract) {
                if let Stmt::Revert(revert) = &stmt {
                    if let Expr::Tuple(tuple) = &revert.expr {
                        if let Some(Expr::Lit(Lit::Str(string))) = tuple.elems.first() {
                            if string.values.len() == 1
                                && string.values[0].value().len() > self.max_length
                            {
                                let location = (string.span().start(), string.span().end());
                                res.push(self.create_diag(file, location, format!("Error message for revert is too long. Should be less than {} characters", self.max_length)));
                            }
                        } else {
                            let location = (
                                revert.revert_token.span().start(),
                                revert.revert_token.span().end(),
                            );
                            res.push(self.create_diag(
                                file,
                                location,
                                "Provide an error message for revert".to_string(),
                            ));
                        }
                    }
                }
                if let Stmt::Expr(expr) = &stmt {
                    if let Expr::Call(call) = &expr.expr {
                        if let Expr::Ident(ref ident) = *(call.expr) {
                            if *ident == "require" || *ident == "assert" {
                                let expr_args = match &call.args.list {
                                    ArgListImpl::Named(_) => continue,
                                    ArgListImpl::Unnamed(args) => args,
                                };

                                if let Some(expr_string) = expr_args.iter().find(|&x| {
                                    if let Expr::Lit(lit) = x {
                                        matches!(
                                            lit,
                                            osmium_libs_solidity_ast_extractor::Lit::Str(_)
                                        )
                                    } else {
                                        false
                                    }
                                }) {
                                    if let Expr::Lit(Lit::Str(lit_str)) = expr_string {
                                        let actual_string = lit_str.values[0].token().to_string();

                                        if actual_string.len() > self.max_length {
                                            let location = (
                                                lit_str.values[0].span().start(),
                                                lit_str.values[0].span().end(),
                                            );
                                            res.push(
                                                self.create_diag(
                                                    file,
                                                    location,
                                                    format!(
                                                        "Error message for revert is too long. Should be less than {} characters",
                                                        self.max_length
                                                    ),
                                                ),
                                            );
                                        }
                                    }
                                } else {
                                    let location = (ident.0.span().start(), ident.0.span().end());
                                    res.push(self.create_diag(
                                        file,
                                        location,
                                        "Provide an error message for revert".to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        res
    }
}

impl ReasonString {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_length = DEFAULT_LENGTH;

        if let Some(data) = &data.data {
            let parsed: Result<usize, serde_json::Error> = serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => max_length = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = ReasonString { max_length, data };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_LENGTH.into()),
        }
    }
}
