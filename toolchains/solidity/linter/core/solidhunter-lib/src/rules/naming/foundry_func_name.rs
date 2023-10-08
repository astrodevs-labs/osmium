use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "foundry-func-name";
const MESSAGE: &str = "Founfry test function name need to respect the convention";

pub struct FoundryFuncName {
    data: RuleEntry,
}

impl FoundryFuncName {
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

impl RuleType for FoundryFuncName {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let re = regex::Regex::new(r"^test(Fork)?(Fuzz)?(Fail)?(_)?(Revert(If_|When_){1})?\w{1,}$").unwrap();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in ast_extractor::retriever::retrieve_functions_nodes(&contract) {
                if let Some(name) = function.name {
                    if !re.is_match(name.as_string().as_str()) {
                        let span = name.span();
                        res.push(self.create_diag((span.start(), span.end()), file));
                    }
                }
            }
        }
        res
    }
}

impl FoundryFuncName {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = FoundryFuncName { data };
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
