use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::{
    retriever::{retrieve_block_nodes, retrieve_contract_nodes},
    Spanned,
};

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "should not be an empty block";
pub const RULE_ID: &str = "empty-block";

pub struct EmptyBlock {
    _data: RuleEntry,
}

impl RuleType for EmptyBlock {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let _reports = check_empty_block(_file);
        for report in _reports.iter() {
            if let Some(report) = report {
                res.push(LintDiag {
                    id: RULE_ID.to_string(),
                    severity: Some(Severity::WARNING),
                    range: report.clone(),
                    code: None,
                    source: None,
                    message: DEFAULT_MESSAGE.to_string(),
                    uri: _file.path.clone(),
                    source_file_content: _file.content.clone(),
                })
            }
        }
        res
    }
}

fn check_empty_block(file: &SolidFile) -> Vec<Option<Range>> {
    let mut res: Vec<Option<Range>> = Vec::new();

    let contracts = retrieve_contract_nodes(&file.data);
    for contract in contracts.iter() {
        if contract.body.is_empty() {
            res.push(Some(Range {
                start: Position {
                    line: contract.span().start().line,
                    character: contract.span().start().column + 1,
                },
                end: Position {
                    line: contract.span().end().line,
                    character: contract.span().end().column,
                },
            }));
        }
    }

    let blocks = retrieve_block_nodes(&file.data);
    for block in blocks.iter() {
        if block.stmts.is_empty() {
            res.push(Some(Range {
                start: Position {
                    line: block.span().start().line,
                    character: block.span().start().column + 1,
                },
                end: Position {
                    line: block.span().end().line,
                    character: block.span().end().column,
                },
            }));
        }
    }
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
