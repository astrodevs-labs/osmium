use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "contract-name-pascalcase";
const MESSAGE: &str = "Contract name need to be in pascal case";

pub struct ContractNamePascalCase {
    data: RuleEntry,
}

impl ContractNamePascalCase {
    fn create_diag(&self, location: Range, file: &SolidFile) -> LintDiag {
        LintDiag {
            id: RULE_ID.to_string(),
            range: location,
            message: MESSAGE.to_string(),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for ContractNamePascalCase {
    fn diagnose(&self, file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            if (contract.name.as_string().chars().nth(0).unwrap() >= 'a'
                        && contract.name.as_string().chars().nth(0).unwrap() <= 'z')
                        || contract.name.as_string().contains('_')
                        || contract.name.as_string().contains('-')
                    {
                        res.push(self.create_diag({
                            let location = contract.name.span();
                            Range {
                                start: Position {
                                    line: location.start().line as u64,
                                    character: location.start().column as u64,
                                },
                                end: Position {
                                    line: location.end().line as u64,
                                    character: location.end().column as u64,
                                },
                            }
                        }, file));
                    }
        }
        res
    }
}

impl ContractNamePascalCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ContractNamePascalCase { data };
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
