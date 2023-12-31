use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "custom-errors";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct CustomErrors {
    data: RuleEntry,
}

impl CustomErrors {
    fn create_diag(
        &self,
        file: &SolidFile,
        location: (LineColumn, LineColumn),
        diag_type: String,
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
            message: format!("Use Custom Errors instead of {} statements", diag_type),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for CustomErrors {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for contract in retriever::retrieve_contract_nodes(&file.data) {
            for stmt in retriever::retrieve_stmts_nodes(&contract) {
                if let Stmt::Revert(revert) = &stmt {
                    if let Expr::Tuple(_) = &revert.expr {
                        let location = (revert.span().start(), revert.expr.span().end());
                        res.push(self.create_diag(file, location, "revert".to_string()));
                    }
                }
                if let Stmt::Expr(expr) = &stmt {
                    if let Expr::Call(call) = &expr.expr {
                        if let Expr::Ident(ref ident) = *(call.expr) {
                            if *ident == "require" || *ident == "assert" {
                                let location = (call.span().start(), call.span().end());
                                res.push(self.create_diag(file, location, ident.to_string()));
                            }
                        }
                    }
                }
            }
        }
        res
    }
}

impl CustomErrors {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = CustomErrors { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: None,
        }
    }
}
