use ast_extractor::retriever::{retrieve_contract_nodes, retrieve_functions_nodes};
use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Visibility modifier not placed first";
pub const RULE_ID: &str = "visibility-modifier-order";

pub struct VisibilityModiferOrder {
    _data: RuleEntry,
}

impl RuleType for VisibilityModiferOrder {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let reports = check_visibility_modifier_order(_file);
        for report in reports {
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
        res
    }
}

fn check_visibility_modifier_order(file: &SolidFile) -> Vec<Range> {
    let mut reports = Vec::new();

    let contracts = retrieve_contract_nodes(&file.data);
    for contract in contracts {
        let functions = retrieve_functions_nodes(&contract);
        for function in functions {
            let mut is_attributes = false;
            function.attributes.iter().for_each(|attributes| {
                if attributes.modifier().is_some() || attributes.mutability().is_some() {
                    is_attributes = true;
                }
                if attributes.visibility().is_some() && is_attributes {
                    reports.push(Range {
                        start: Position {
                            line: attributes.span().start().line,
                            character: attributes.span().start().column,
                        },
                        end: Position {
                            line: attributes.span().end().line,
                            character: attributes.span().end().column,
                        },
                    });
                }
            });
        }
    }
    reports
}

impl VisibilityModiferOrder {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = VisibilityModiferOrder { _data: data };
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
