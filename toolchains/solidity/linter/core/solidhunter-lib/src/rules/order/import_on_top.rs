use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{decode_location, CodeLocation, SourceUnitChildNodes};

pub struct ImportOnTop {
    data: RuleEntry,
}

impl ImportOnTop {
    fn create_diag(&self, file: &SolidFile, location: (CodeLocation, CodeLocation)) -> LintDiag {
        LintDiag {
            id: "import-on-top".to_string(),
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
            message: String::from("Import must be on top in the file"),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for ImportOnTop {
    fn diagnose(&self, file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut last_import_location = 0;

        for i in 1..file.data.nodes.len() {
            match &file.data.nodes[i] {
                SourceUnitChildNodes::ImportDirective(_) => {
                    last_import_location = i;
                }
                _ => {
                    break;
                }
            }
        }

        for i in 1..file.data.nodes.len() {
            match &file.data.nodes[i] {
                SourceUnitChildNodes::ImportDirective(import) => {
                    if i > last_import_location {
                        let location = decode_location(&import.src, &file.content);
                        res.push(self.create_diag(file, location));
                    }
                }
                _ => {}
            }
        }

        res
    }
}

impl ImportOnTop {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ImportOnTop { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "import-on-top".to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}
