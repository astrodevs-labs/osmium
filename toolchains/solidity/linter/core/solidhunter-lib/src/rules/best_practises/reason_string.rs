use ast_extractor::*;

use crate::linter::SolidFile;
use crate::rules::types::{RuleEntry, RuleType};
use crate::types::{LintDiag, Position, Range, Severity};

pub const RULE_ID: &str = "reason-string";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

// Specific
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
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for ReasonString {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for call_expr in retriever::retrieve_expr_call_nodes(&file.data) {
            let expr_require = match *call_expr.expr {
                Expr::Ident(require_ident) => require_ident,
                _ => continue,
            };

            if expr_require.as_string() != "require" && expr_require.as_string() != "revert" {
                continue;
            }

            let expr_args = match call_expr.args.list {
                ArgListImpl::Named(_) => continue,
                ArgListImpl::Unnamed(args) => args,
            };

            if let Some(expr_string) = expr_args.iter().find(|&x| {
                if let Expr::Lit(lit) = x {
                    matches!(lit, ast_extractor::Lit::Str(_))
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
                                    "reason-string: A revert statement must have a reason string of length less than {}",
                                    self.max_length
                                ),
                            ),
                        );
                    }
                }
            } else {
                let location = (expr_require.0.span().start(), expr_require.0.span().end());
                res.push(self.create_diag(
                    file,
                    location,
                    "reason-string: A require statement must have a reason string".to_string(),
                ));
            }
        }

        res
    }
}

impl ReasonString {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_length = DEFAULT_LENGTH;

        if !data.data.is_empty() {
            max_length = match data.data[0].as_u64() {
                Some(val) => val as usize,
                None => DEFAULT_LENGTH,
            };
        }

        let rule = ReasonString { max_length, data };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: vec![serde_json::Value::String(DEFAULT_LENGTH.to_string())],
        }
    }
}
