use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Fallback should contains payable attributs";
pub const RULE_ID: &str = "payable-fallback";

pub struct PayableFallback {
    _data: RuleEntry,
}

impl RuleType for PayableFallback {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let _report = check_fallback_payable(_file);

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

fn check_fallback_payable(file: &SolidFile) -> Option<Range> {
    let mut res: Option<Range> = None;
    let mut line_index = 1;

    file.content.lines().for_each(|line| {
        let fallback_index = line.find("fallback");
        if fallback_index.is_some() {
            if !line.find("payable").is_some() {
                res = Some(Range {
                    start: Position {
                        line: line_index,
                        character: fallback_index.unwrap() + 1,
                    },
                    end: Position {
                        line: line_index,
                        character: fallback_index.unwrap() + 1 + "payable".len(),
                    },
                })
            }
        }
        line_index += 1;
    });
    res
}

impl PayableFallback {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = PayableFallback { _data: data };
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
