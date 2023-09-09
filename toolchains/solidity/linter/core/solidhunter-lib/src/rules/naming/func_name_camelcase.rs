use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{decode_location, ContractDefinitionChildNodes, FunctionDefinitionKind, SourceUnitChildNodes, CodeLocation};

pub struct FuncNameCamelCase {
    data: RuleEntry,
}

impl RuleType for FuncNameCamelCase {
    fn create_diag(&self, location: (CodeLocation, CodeLocation), file: SolidFile) {
        LintDiag {
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
            message: format!("Function name need to be in camel case"),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }

    fn diagnose(&self, file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for node in &file.data.nodes {
            match node {
                SourceUnitChildNodes::ContractDefinition(contract) => {
                    for node in &contract.nodes {
                        match node {
                            ContractDefinitionChildNodes::FunctionDefinition(function) => {
                                if function.kind != FunctionDefinitionKind::Constructor
                                    && (!(function.name.chars().nth(0).unwrap_or(' ') >= 'a'
                                        && function.name.chars().nth(0).unwrap_or(' ') <= 'z')
                                        || function.name.contains('_')
                                        || function.name.contains('-'))
                                {
                                    //Untested
                                    let location = decode_location(
                                        function.name_location.as_ref().unwrap(),
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
                }
                _ => {
                    continue;
                }
            }
        }
        res
    }
}

impl FuncNameCamelCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = FuncNameCamelCase { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "func-name-camelcase".to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}
