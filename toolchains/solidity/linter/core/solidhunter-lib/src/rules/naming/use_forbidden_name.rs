use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::*;

pub const RULE_ID: &str = "use-forbidden-name";

pub struct UseForbiddenName {
    data: RuleEntry,
}

impl UseForbiddenName {
    fn create_diag(
        &self,
        file: &SolidFile,
        location: (LineColumn, LineColumn),
        name: &String,
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
            message: format!("Forbidden variable name: {}", name),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for UseForbiddenName {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let blacklist = ['I', 'l', 'O'];

        let contracts = retriever::retrieve_contract_nodes(&file.data);

        // var def => contract def
        for contract in contracts.iter() {
            for node_var in contract.body.iter() {
                let var = match node_var {
                    Item::Variable(var) => var,
                    _ => continue,
                };
                if var.name.as_string().len() == 1
                    && blacklist.contains(&var.name.as_string().chars().next().unwrap())
                {
                    let location = (var.name.span().start(), var.name.span().end());
                    res.push(self.create_diag(file, location, &var.name.as_string()));
                }
            }
        }
        res
    }
}

impl UseForbiddenName {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = UseForbiddenName { data };
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
