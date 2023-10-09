use ast_extractor::Spanned;
use serde_json::Value;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "foundry-func-name";
const MESSAGE: &str = "Founfry test function name need to respect the convention";

pub struct FoundryFuncName {
    data: RuleEntry,
    excluded: Vec<String>,
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
        if !file.path.ends_with(".t.sol") {
            return vec![];
        }
        let mut res = Vec::new();
        let re = regex::Regex::new(r"^test(Fork)?(Fuzz)?(Fail)?(_)?(Revert(If_|When_){1})?\w{1,}$")
            .unwrap();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in ast_extractor::retriever::retrieve_functions_nodes(&contract) {
                let visibility = function
                    .attributes
                    .iter()
                    .find(|attr| matches!(attr, ast_extractor::FunctionAttribute::Visibility(_)));
                let visibility = match visibility {
                    Some(ast_extractor::FunctionAttribute::Visibility(visibility)) => visibility,
                    _ => continue,
                };

                if !matches!(visibility, ast_extractor::Visibility::Public(_))
                    && !matches!(visibility, ast_extractor::Visibility::External(_))
                {
                    continue;
                }
                if let Some(name) = function.name {
                    if !re.is_match(&name.as_string()) && !self.excluded.contains(&name.as_string())
                    {
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
        let mut excluded: Vec<String> = Vec::new();
        data.data.iter().for_each(|value| {
            if let Value::String(val) = value {
                excluded.push(val.to_string());
            } else {
                eprintln!("Invalid value for rule foundry-func-name: {:?}", value);
            }
        });
        let rule = FoundryFuncName { excluded, data };
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
