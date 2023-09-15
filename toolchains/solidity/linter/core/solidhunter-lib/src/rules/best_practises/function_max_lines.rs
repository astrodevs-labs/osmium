use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Function contains too much lines";

// specific
pub const DEFAULT_MAX_LINES: usize = 20;

pub struct FunctionMaxLines {
    number_max_lines: usize,
    _data: RuleEntry,
}

impl RuleType for FunctionMaxLines {
    fn diagnose(&self, _file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let functions = get_all_functions_from_ast(&_file.data);
        for function in functions {
            let _report = check_function_lines(_file, &function, self.number_max_lines);
            if let Some(report) = _report {
                res.push(LintDiag {
                    id: Self::RULE_ID.to_string(),
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
        res
    }
}

// returns a struct containing the line number of the start and end of the function if it is too long
fn check_function_lines(
    file: &SolidFile,
    function: &ast_extractor::ItemFunction,
    nb_max_line: usize,
) -> Option<Range> {
    let mut res: Option<Range> = None;
    let start_span = function.span().start();
    let index = crate::rules::utils::absolute_index_from_location(start_span, &file.content);
    let mut function_lines: usize = 0;
    let mut left_bracket: usize = 0;
    let mut right_bracket: usize = 0;
    let mut last_bracket_line: usize = 0;

    for (_, c) in file.content.chars().enumerate().skip(index) {
        if c == '{' {
            left_bracket += 1;
        }
        if c == '}' {
            right_bracket += 1;
        }
        if c == '\n' {
            function_lines += 1;
        }
        if right_bracket > 0 && left_bracket == right_bracket {
            last_bracket_line = start_span.line + function_lines;
            break;
        }
    }
    if function_lines > nb_max_line {
        res = Some(Range {
            start: Position {
                line: start_span.line as u64,
                character: start_span.column as u64,
            },
            end: Position {
                line: last_bracket_line as u64,
                character: 1,
            },
        });
    }
    res
}

fn get_all_functions_from_ast(ast_nodes: &ast_extractor::File) -> Vec<ast_extractor::ItemFunction> {
    let mut res = Vec::new();
    let contract = ast_nodes
        .items
        .iter()
        .filter_map(|item| match item {
            ast_extractor::Item::Contract(contract) => Some(contract),
            _ => None,
        })
        .next();

    if let Some(contract) = contract {
        res = ast_extractor::retriever::retrieve_functions_nodes(contract);
    }
    res
}

impl FunctionMaxLines {
    pub(crate) const RULE_ID: &'static str = "function-max-lines";

    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_number_lines = DEFAULT_MAX_LINES;
        
        if data.data.len() > 0 {
            max_number_lines = match data.data[0].parse::<usize>() {
                Ok(v) => v,
                Err(_) => DEFAULT_MAX_LINES,
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
            id: FunctionMaxLines::RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![DEFAULT_MAX_LINES.to_string()],
        }
    }
}

