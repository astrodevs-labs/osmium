use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "should not be an empty block";
pub const RULE_ID: &str = "empty-block";

pub struct EmptyBlock {
    _data: RuleEntry,
}

impl RuleType for EmptyBlock {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let _report = check_empty_block(_file);
        if let Some(report) = _report {
            res.push(LintDiag {
                id: RULE_ID.to_string(),
                severity: Some(Severity::WARNING),
                range: report,
                code: None,
                source: None,
                message: DEFAULT_MESSAGE.to_string(),
                uri: _file.path.clone(),
                source_file_content: _file.content.clone(),
            })
        }
        print!("{:?}", res);
        println!("");
        res
    }
}

fn check_empty_block(file: &SolidFile) -> Option<Range> {
    let mut res: Option<Range> = None;
    let mut line_index = 1;
    let mut current_index = 1;

    file.content.lines().for_each(|line| {
        let left_bracket_index = line.find("{");
        if left_bracket_index.is_some() {
            let mut index_in_line = 1;
            for (_, c) in file
                .content
                .chars()
                .enumerate()
                .skip(current_index + left_bracket_index.unwrap())
            {
                if (c != ' ') && (c != '\n') && (c != '}') {
                    break;
                }
                if c == '\n' {
                    line_index += 1;
                    index_in_line = 0;
                }
                if c == '}' {
                    res = Some(Range {
                        start: Position {
                            line: line_index,
                            character: left_bracket_index.unwrap() + 1,
                        },
                        end: Position {
                            line: line_index,
                            character: left_bracket_index.unwrap() + 1 + index_in_line,
                        },
                    });
                    break;
                }
                index_in_line += 1;
            }
        }
        line_index += 1;
        current_index += line.len() + 1;
    });
    res
}

impl EmptyBlock {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = EmptyBlock { _data: data };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}
