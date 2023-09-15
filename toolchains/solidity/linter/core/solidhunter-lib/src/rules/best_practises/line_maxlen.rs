use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "line-max-len";

const DEFAULT_LENGTH: u32 = 80;

pub struct LineMaxLen {
    max_len: usize,
    data: RuleEntry,
}

impl LineMaxLen {
    fn create_diag(&self, file: &SolidFile, line_idx: usize, line: &str) -> LintDiag {
        LintDiag {
            range: Range {
                start: Position {
                    line: line_idx as u64,
                    character: self.max_len as u64,
                },
                end: Position {
                    line: line_idx as u64,
                    character: line.len() as u64,
                },
            },
            id: RULE_ID.to_string(),
            message: format!("Line is too long: {}", line.len()),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for LineMaxLen {
    fn diagnose(&self, file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut line_idx = 1;

        for line in file.content.lines() {
            if line.len() > self.max_len {
                res.push(self.create_diag(file, line_idx, line));
            }
            line_idx += 1;
        }
        res
    }
}

impl LineMaxLen {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = LineMaxLen {
            max_len: data.data[0].parse::<usize>().unwrap(),
            data,
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![DEFAULT_LENGTH.to_string()],
        }
    }
}
