use ast_extractor::*;

use crate::linter::SolidFile;
use crate::rules::types::{RuleEntry, RuleType};
use crate::types::{LintDiag, Position, Range, Severity};

pub const RULE_ID: &str = "no-console";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct NoConsole {
    data: RuleEntry,
}

impl NoConsole {
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

impl RuleType for NoConsole {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res: Vec<LintDiag> = Vec::new();

        // Check functions calls
        for expr_member in retriever::retrieve_expr_member_nodes(&file.data) {
            if let Expr::Ident(expr_ident) = *expr_member.expr {
                if expr_ident == "console" {
                    if let Expr::Call(expr_call) = *expr_member.member {
                        if let Expr::Ident(expr_ident) = *expr_call.expr {
                            if expr_ident.as_string().starts_with("log") {
                                let diag = self.create_diag(
                                    file,
                                    (expr_ident.span().start(), expr_ident.span().end()),
                                    format!("{}: console.log(...) is forbidden", RULE_ID),
                                );
                                res.push(diag);
                            }
                        }
                    }
                }
            }
        }

        // Check imports
        let blacklist: Vec<String> = vec![
            "hardhat/console.sol".to_string(),
            "forge-std/console".to_string(), // console?.sol (easier to blacklist this way)
        ];

        for header in retriever::retrieve_import_directive_nodes(&file.data) {
            if let ImportPath::Plain(pathplain) = &header.path {
                for test in &pathplain.path.values {
                    for refused in blacklist.clone() {
                        if test.value().contains(&refused) {
                            let diag = self.create_diag(
                                file,
                                (header.span().start(), header.span().end()),
                                format!("{}: forbidden usage of console headers", RULE_ID),
                            );
                            res.push(diag);
                        }
                    }
                }
            }
        }

        res
    }
}

impl NoConsole {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NoConsole { data };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: vec![],
        }
    }
}
