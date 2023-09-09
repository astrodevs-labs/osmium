use solc_wrapper::ast::utils::{self, get_all_nodes_by_type};
use solc_wrapper::{decode_location, Expression, NodeType};

use crate::linter::SolidFile;
use crate::rules::types::{RuleEntry, RuleType};
use crate::types::{LintDiag, Position, Range, Severity};

pub const RULE_ID: &str = "reason-string";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

// Specific
const DEFAULT_LENGTH: u32 = 32;

pub struct ReasonString {
    max_length: u32,
    data: RuleEntry,
}

impl RuleType for ReasonString {

    fn create_diag(&self, file: &SolidFile, location: (CodeLocation, CodeLocation), message: String) -> LintDiag {
        LintDiag {
            range: Range {
                start: Position {
                    line: location.0.line as u64,
                    character: location.0.column as u64,
                },
                end: Position {
                    line: location.1.line as u64,
                    character: location.1.column as u64,
                },
                length: location.0.length as u64,
            },
            message: message,
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }

    fn diagnose(&self, file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let nodes = get_all_nodes_by_type(file.data.clone(), NodeType::FunctionCall);
        for i in &nodes {
            match i {
                utils::Nodes::FunctionCall(j) => match &j.expression {
                    Expression::Identifier(v) => {
                        if v.name == "require" {
                            if j.arguments.len() != 2 {
                                let location = decode_location(&j.src, &file.content);
                                res.push(self.create_diag(file, location, format!("reason-string: A require statement must have a reason string")));
                            } else {
                                for nj in &j.arguments {
                                    match nj {
                                        Expression::Literal(z) => {
                                            if z.value.clone().unwrap().len()
                                                > self.max_length as usize
                                            {
                                                let location =
                                                    decode_location(&z.src, &file.content);
                                                res.push(self.create_diag(file, location, format!("reason-string: A revert statement must have a reason string of length less than {}", self.max_length)));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        } else if v.name == "revert" {
                            if j.arguments.len() == 0 {
                                let location = decode_location(&j.src, &file.content);
                                res.push(self.create_diag(file, location, format!("reason-string: A revert statement must have a reason string")));
                            } else {
                                match &j.arguments[0] {
                                    Expression::Literal(z) => {
                                        if z.value.clone().unwrap().len() > self.max_length as usize
                                        {
                                            let location = decode_location(&z.src, &file.content);
                                            res.push(self.create_diag(file, location, format!("reason-string: A revert statement must have a reason string of length less than {}", self.max_length)));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        res
    }
}

impl ReasonString {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = ReasonString {
            max_length: data.data[0].parse::<u32>().unwrap(),
            data,
        };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: vec![DEFAULT_LENGTH.to_string()],
        }
    }
}
