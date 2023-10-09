use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::*;

pub const RULE_ID: &str = "not-rely-on-time";
const MESSAGE: &str = "Avoid making time-based decisions in your business logic";

pub struct NotRelyOnTime {
    data: RuleEntry,
}

impl NotRelyOnTime {
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

impl RuleType for NotRelyOnTime {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut i = 1;

        for line in file.content.lines() {
            if let Some(index) = line.find("now") {
                res.push(self.create_diag(
                    (
                        LineColumn {
                            line: i,
                            column: index,
                        },
                        LineColumn {
                            line: i,
                            column: index + 3,
                        },
                    ),
                    file,
                ));
            }
            if let Some(index) = line.find("block.timestamp") {
                res.push(self.create_diag(
                    (
                        LineColumn {
                            line: i,
                            column: index,
                        },
                        LineColumn {
                            line: i,
                            column: index + 15,
                        },
                    ),
                    file,
                ));
            }
            i += 1;
        }
        res
    }
}

impl NotRelyOnTime {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NotRelyOnTime { data };
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
