use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::{LineColumn, Spanned};

// global
pub const RULE_ID: &str = "func-param-name-mixedcase";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Function param name must be in mixedCase";

pub struct FuncParamNameMixedCase {
    data: RuleEntry,
}

impl FuncParamNameMixedCase {
    fn create_diag(&self, location: (LineColumn, LineColumn), file: &SolidFile) -> LintDiag {
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
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for FuncParamNameMixedCase {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in ast_extractor::retriever::retrieve_functions_nodes(&contract) {
                for arg in function.arguments.iter() {
                    if let Some(name) = &arg.name {
                        if !(name.as_string().chars().next().unwrap() >= 'a'
                            && name.as_string().chars().next().unwrap() <= 'z')
                            || name.as_string().contains('_')
                            || name.as_string().contains('-')
                        {
                            let span = name.span();
                            res.push(self.create_diag((span.start(), span.end()), file));
                        }
                    }
                }
            }
        }
        res
    }
}

impl FuncParamNameMixedCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = FuncParamNameMixedCase { data };
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
