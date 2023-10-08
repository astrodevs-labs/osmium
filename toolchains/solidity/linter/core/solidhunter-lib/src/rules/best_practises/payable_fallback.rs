use ast_extractor::retriever::{retrieve_contract_nodes, retrieve_functions_nodes};
use ast_extractor::{FunctionKind, ItemFunction, Mutability, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Fallback should contains payable attributes";
pub const RULE_ID: &str = "payable-fallback";

pub struct PayableFallback {
    _data: RuleEntry,
}

impl RuleType for PayableFallback {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let reports = check_fallback_payable(_file);

        for report in reports {
            if let Some(rep) = report {
                res.push(LintDiag {
                    id: RULE_ID.to_string(),
                    severity: Some(Severity::WARNING),
                    range: rep,
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

fn check_fallback_payable(file: &SolidFile) -> Vec<Option<Range>> {
    let mut res: Vec<Option<Range>> = Vec::new();

    let contracts = retrieve_contract_nodes(&file.data);
    for contract in contracts {
        let functions = retrieve_functions_nodes(&contract);

        for function in functions {
            if FunctionKind::is_fallback(function.kind) {
                res = check_attribute(res, function);
            } else if function.name.is_none() {
                res = check_attribute(res, function);
            }
        }
    }
    res
}
fn check_attribute(mut res: Vec<Option<Range>>, function: ItemFunction) -> Vec<Option<Range>> {
    let mut is_payable = false;
    for attributes in function.attributes.iter() {
        if attributes.mutability().is_some()
            && Mutability::is_payable(attributes.mutability().unwrap())
        {
            is_payable = true;
        }
    }
    if !is_payable {
        res.push(create_report(function));
    }
    res
}

fn create_report(function: ItemFunction) -> Option<Range> {
    let res = Some(Range {
        start: Position {
            line: function.attributes.span().start().line,
            character: function.attributes.span().start().column + 1,
        },
        end: Position {
            line: function.attributes.span().end().line,
            character: function.attributes.span().end().column,
        },
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
