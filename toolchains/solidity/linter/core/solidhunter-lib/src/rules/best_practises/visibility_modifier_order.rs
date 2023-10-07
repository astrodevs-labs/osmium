use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Visibility modifier not placed first";
pub const RULE_ID: &str = "visibility-modifier-order";

pub struct VisibilityModiferOrder {
    _data: RuleEntry,
}

// je retourne une erreur si la visibility modifier
// n'est pas juste apres nom_de_fonction()

impl RuleType for VisibilityModiferOrder {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut line_index = 1;

        _file.content.lines().for_each(|line| {
            if line.find("function").is_some() {
                let line_splitted = line.split_whitespace().collect::<Vec<&str>>();
                if line_splitted[2] != "public" && line_splitted[2] != "private" {
                    let start_index = line.find(line_splitted[2]).unwrap();
                    let range = Range {
                        start: Position {
                            line: line_index,
                            character: start_index + 1,
                        },
                        end: Position {
                            line: line_index,
                            character: start_index + line_splitted[2].len(),
                        },
                    };
                    res.push(LintDiag {
                        id: RULE_ID.to_string(),
                        range: range,
                        severity: Some(Severity::WARNING),
                        code: None,
                        source: None,
                        message: DEFAULT_MESSAGE.to_string(),
                        uri: _file.path.clone(),
                        source_file_content: _file.content.clone(),
                    });
                }
            }
            line_index += 1;
        });
        println!("res {:?}", res);
        res
    }
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
