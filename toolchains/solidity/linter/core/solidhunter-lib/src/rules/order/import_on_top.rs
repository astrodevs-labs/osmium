use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub struct ImportOnTop {
    data: RuleEntry,
}

impl ImportOnTop {
    fn create_diag(&self, file: &SolidFile, location: (ast_extractor::LineColumn, ast_extractor::LineColumn)) -> LintDiag {
        let mut range = Range {
            start: Position {
                line: location.0.line as u64,
                character: location.0.column as u64,
            },
            end: Position {
                line: location.1.line as u64,
                character: location.1.column as u64,
            },
            length: 0,
        };
        range.compute_length(&file.content);
        LintDiag {
            id: "import-on-top".to_string(),
            range,
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

        for i in 1..file.data.items.len() {
            match &file.data.items[i] {
                ast_extractor::Item::Import(_) => {
                    last_import_location = i;
                }
                _ => {
                    break;
                }
            }
        }

        for i in 1..file.data.items.len() {
            match &file.data.items[i] {
                ast_extractor::Item::Import(import) => {
                    if i > last_import_location {
                        let location = (import.span().start(), import.span().end());
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
