use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "max-states-count";

// specific
const DEFAULT_MAX_STATES: usize = 15;
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

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
            message: format!(
                "Contract has {} states declarations but allowed no more than {}",
                count, self.max_states
            ),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
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

        if let Some(data) = &data.data {
            let parsed: Result<usize, serde_json::Error> = serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => max_states = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = MaxStatesCount { max_states, data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_MAX_STATES.into()),
        }
    }
}
