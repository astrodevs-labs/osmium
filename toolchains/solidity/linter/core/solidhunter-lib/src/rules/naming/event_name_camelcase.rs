use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "event-name-camelcase";
const MESSAGE: &str = "Event name must be in CamelCase";

pub struct EventNameCamelCase {
    data: RuleEntry,
}

impl EventNameCamelCase {
    fn create_diag(
        &self,
        location: (ast_extractor::LineColumn, ast_extractor::LineColumn),
        file: &SolidFile,
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
            message: MESSAGE.to_string(),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

fn is_camel_case(name: &str) -> bool {
    if !(name.chars().next().unwrap_or(' ') >= 'A' && name.chars().next().unwrap_or(' ') <= 'Z') {
        return false;
    }
    if name.contains('_') || name.contains('-') {
        return false;
    }
    true
}

impl RuleType for EventNameCamelCase {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for event in ast_extractor::retriever::retrieve_events_file_nodes(&file.data) {
            if !is_camel_case(&event.name.to_string()) {
                let span = event.name.span();
                res.push(self.create_diag((span.start(), span.end()), file));
            }
        }

        for contract in contracts {
            for event in ast_extractor::retriever::retrieve_events_contract_nodes(&contract) {
                if !is_camel_case(&event.name.to_string()) {
                    let span = event.name.span();
                    res.push(self.create_diag((span.start(), span.end()), file));
                }
            }
        }
        res
    }
}

impl EventNameCamelCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = EventNameCamelCase { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}
