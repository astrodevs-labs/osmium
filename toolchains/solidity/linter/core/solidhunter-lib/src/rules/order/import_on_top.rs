use ast_extractor::{LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "import-on-top";

// specific
const DEFAULT_MESSAGE: &str = "Import statements must be on top";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct ImportOnTop {
    data: RuleEntry,
}

impl ImportOnTop {
    fn create_diag(&self, file: &SolidFile, location: (LineColumn, LineColumn)) -> LintDiag {
        let range = Range {
            start: Position {
                line: location.0.line,
                character: location.0.column,
            },
            end: Position {
                line: location.1.line,
                character: location.1.column,
            },
        };
        LintDiag {
            id: RULE_ID.to_string(),
            range,
            message: DEFAULT_MESSAGE.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for ImportOnTop {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut last_import_location = 0;

        for i in 0..file.data.items.len() {
            match &file.data.items[i] {
                ast_extractor::Item::Pragma(_) => {
                    continue;
                }
                ast_extractor::Item::Import(_) => {
                    last_import_location = i;
                }
                _ => {
                    break;
                }
            }
        }

        for i in 0..file.data.items.len() {
            if let ast_extractor::Item::Import(import) = &file.data.items[i] {
                if i > last_import_location {
                    let location = (import.span().start(), import.span().end());
                    res.push(self.create_diag(file, location));
                }
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
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: None,
        }
    }
}
