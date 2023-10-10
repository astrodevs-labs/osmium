use ast_extractor::{Spanned, FunctionBody};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Function contains too much lines";
pub const RULE_ID: &str = "function-max-lines";

// specific
pub const DEFAULT_MAX_LINES: usize = 20;

pub struct FunctionMaxLines {
    number_max_lines: usize,
    _data: RuleEntry,
}

impl RuleType for FunctionMaxLines {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        for contract in ast_extractor::retriever::retrieve_contract_nodes(&_file.data) {
            for function in ast_extractor::retriever::retrieve_functions_nodes(&contract) {
                let report = check_function_lines(&function, self.number_max_lines);
                if let Some(report) = report {
                    res.push(LintDiag {
                        id: RULE_ID.to_string(),
                        range: report,
                        severity: Some(Severity::WARNING),
                        code: None,
                        source: None,
                        message: DEFAULT_MESSAGE.to_string(),
                        uri: _file.path.clone(),
                        source_file_content: _file.content.clone(),
                    });
                }
            }
        }
        res
    }
}

// returns a struct containing the line number of the start and end of the function if it is too long
fn check_function_lines(
    function: &ast_extractor::ItemFunction,
    nb_max_line: usize,
) -> Option<Range> {
    if let FunctionBody::Block(block) = &function.body {
        let line_diff = block.span().end().line - block.span().start().line;
        let start_span = function.name.span().start();
        let end_span = block.span().end();
        if line_diff > nb_max_line {
            return Some(Range {
                start: Position {
                    line: start_span.line,
                    character: start_span.column,
                },
                end: Position {
                    line: end_span.line,
                    character: end_span.column,
                },
            })
        }
    }
    None
}

impl FunctionMaxLines {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_number_lines = DEFAULT_MAX_LINES;

        if !data.data.is_empty() {
            max_number_lines = match data.data[0].as_u64() {
                Some(v) => v as usize,
                None => DEFAULT_MAX_LINES,
            };
        }
        let rule = FunctionMaxLines {
            number_max_lines: max_number_lines,
            _data: data,
        };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![serde_json::Value::String(DEFAULT_MAX_LINES.to_string())],
        }
    }
}
