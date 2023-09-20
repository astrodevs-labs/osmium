use ast_extractor::*;

use crate::linter::SolidFile;
use crate::rules::types::{RuleEntry, RuleType};
use crate::types::{LintDiag, Position, Range, Severity};

pub const RULE_ID: &str = "reason-string";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

// Specific
const DEFAULT_LENGTH: u32 = 32;

pub struct ReasonString {
    max_length: u32,
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
                    line: location.0.line as u64,
                    character: location.0.column as u64,
                },
                end: Position {
                    line: location.1.line as u64,
                    character: location.1.column as u64,
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

fn get_call_expressions(ast_nodes: &ast_extractor::File) -> Vec<ExprCall> {
    let mut res = Vec::new();
    let mut calls: Vec<ExprCall> = Vec::new();
    let contract = ast_nodes
        .items
        .iter()
        .filter_map(|item| match item {
            ast_extractor::Item::Contract(contract) => Some(contract),
            _ => None,
        })
        .next();

    if let Some(contract) = contract {
        res = ast_extractor::retriever::retrieve_functions_nodes(contract);
    }
    for func in res {
        if let FunctionBody::Block(fn_body) = func.body {
            for stmt in fn_body.stmts {
                if let Stmt::Expr(stmt_expr) = stmt {
                    if let Expr::Call(call_expr) = stmt_expr.expr {
                        calls.push(call_expr);
                    }
                }

            }
        }
    }
    calls
}

impl RuleType for ReasonString {
    fn diagnose(&self, file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let calls = get_call_expressions(&file.data);

        for call_expr in calls {
            let expr_require = match *call_expr.expr {
                Expr::Ident(require_ident) => require_ident,
                _ => continue,
            };

            if expr_require.as_string() != "require"
                && expr_require.as_string() != "revert"
            {
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
                if let Expr::Lit(lit_string) = expr_string {
                    if let ast_extractor::Lit::Str(lit_str) = lit_string {
                        let actual_string = lit_str.values[0].token().to_string();

                        if actual_string.len() > self.max_length as usize {
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
                }
            } else {
                let location =
                    (expr_require.0.span().start(), expr_require.0.span().end());
                res.push(
                    self.create_diag(
                        file,
                        location,
                        "reason-string: A require statement must have a reason string".to_string(),
                    ),
                );
            }
        }

        res
    }
}

impl ReasonString {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ReasonString {
            max_length: data.data[0].parse::<u32>().unwrap(),
            data,
        };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: vec![DEFAULT_LENGTH.to_string()],
        }
    }
}
