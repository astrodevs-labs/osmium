use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::*;

pub const RULE_ID: &str = "max-states-count";
const DEFAULT_MAX_STATES: usize = 15;

pub struct MaxStatesCount {
    max_states: usize,
    data: RuleEntry,
}

impl MaxStatesCount {
    fn create_diag(
        &self,
        file: &SolidFile,
        location: (LineColumn, LineColumn),
        count: usize,
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
            message: format!("Too many states: {}", count),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for MaxStatesCount {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let mut count = 0;
        let contracts = retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts.iter() {
            for node_var in contract.body.iter() {
                let var = match node_var {
                    Item::Variable(var) => var,
                    _ => continue,
                };
                count += 1;
                if count > self.max_states {
                    let location = (var.span().start(), var.span().end());
                    res.push(self.create_diag(file, location, count));
                }
            }
        }
        res
    }
}

impl MaxStatesCount {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut max_states = DEFAULT_MAX_STATES;
        if !data.data.is_empty() {
            max_states = match data.data[0].parse::<usize>() {
                Ok(v) => v,
                Err(_) => DEFAULT_MAX_STATES,
            };
        }
        let rule = MaxStatesCount { max_states, data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec!["15".to_string()],
        }
    }
}
