use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "no-inline-assembly";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Avoid to use inline assembly. It is acceptable only in rare cases";

pub struct NoInlineAssembly {
    data: RuleEntry,
}

impl NoInlineAssembly {
    fn create_diag(&self, file: &SolidFile, location: (LineColumn, LineColumn)) -> LintDiag {
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
            message: DEFAULT_MESSAGE.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for NoInlineAssembly {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for contract in retriever::retrieve_contract_nodes(&file.data) {
            for stmt in retriever::retrieve_stmts_nodes(&contract) {
                if let Stmt::Assembly(_) = stmt {
                    let location = (stmt.span().start(), stmt.span().end());
                    res.push(self.create_diag(file, location));
                }
            }
        }
        res
    }
}

impl NoInlineAssembly {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NoInlineAssembly { data };
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
