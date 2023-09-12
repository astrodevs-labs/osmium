use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{decode_location, CodeLocation, SourceUnitChildNodes};

pub struct ContractNamePascalCase {
    data: RuleEntry,
}

impl ContractNamePascalCase {
    fn create_diag(&self, location: (CodeLocation, CodeLocation), file: &SolidFile) -> LintDiag {
        LintDiag {
            id: "contract-name-pascalcase".to_string(),
            range: Range {
                start: Position {
                    line: location.0.line as u64,
                    character: location.0.column as u64,
                },
                end: Position {
                    line: location.1.line as u64,
                    character: location.1.column as u64,
                },
                length: location.0.length as u64,
            },
            message: "Contract name need to be in pascal case".to_string(),
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

        for node in &file.data.nodes {
            match node {
                SourceUnitChildNodes::ContractDefinition(contract) => {
                    if (contract.name.chars().nth(0).unwrap() >= 'a'
                        && contract.name.chars().nth(0).unwrap() <= 'z')
                        || contract.name.contains('_')
                        || contract.name.contains('-')
                    {
                        //Untested
                        let location = decode_location(
                            contract.name_location.as_ref().unwrap(),
                            &file.content,
                        );
                        res.push(self.create_diag(location, file));
                    }
                }
                _ => {
                    continue;
                }
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
            id: "contract-name-pascalcase".to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}
