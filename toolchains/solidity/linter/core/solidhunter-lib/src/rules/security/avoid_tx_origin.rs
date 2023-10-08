use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::*;

pub const RULE_ID: &str = "avoid-tx-origin";
const MESSAGE: &str = "Avoid to use tx.origin";

struct ExprVisitor {
    exprs: Vec<ExprMember>,
}

impl ExprVisitor {
    pub fn new() -> Self {
        Self {
            exprs: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ExprVisitor {
    fn visit_expr_member(&mut self,i: &'ast ExprMember) {
        self.exprs.push(i.clone());
        visit::visit_expr_member(self, i);
    }
}

pub struct AvoidTxOrigin {
    data: RuleEntry,
}

impl AvoidTxOrigin {
    fn create_diag(
        &self,
        location: (LineColumn, LineColumn),
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

impl RuleType for AvoidTxOrigin {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = ExprVisitor::new();
        for contract in ast_extractor::retriever::retrieve_contract_nodes(&file.data) {
            visitor.visit_item_contract(&contract);
        }

        for expr in visitor.exprs {
            if let Expr::Ident(ident) = &*expr.expr {
                if let Expr::Ident(ident2) = &*expr.member {
                    if ident == "tx" && ident2 == "origin" {
                        let location = (expr.span().start(), expr.span().end());
                        res.push(self.create_diag(location, file));
                    }
                }
            }
        }
        res
    }
}

impl AvoidTxOrigin {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = AvoidTxOrigin { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}