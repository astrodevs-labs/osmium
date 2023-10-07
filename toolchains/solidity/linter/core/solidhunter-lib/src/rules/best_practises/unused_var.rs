use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "should use variable or remove it";
pub const RULE_ID: &str = "unused-var";

pub struct UnusedVar {
    _data: RuleEntry,
}

impl RuleType for UnusedVar {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let _report = check_unused_var(_file);
        //fonction qui return un report
        if let Some(report) = _report {
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
        println!("res: {:?}", res);
        res
    }
}

fn check_unused_var(file: &SolidFile) -> Option<Range> {
    let mut report: Option<Range> = None;
    let mut line_index = 1;

    file.content.lines().for_each(|line| {
        // let is_in_function = false;
        let index_equal = line.find("=");
        if index_equal.is_some() {
            let var_declaration = &line[..index_equal.unwrap()];
            let parts = var_declaration.split(" ");
            let mut var_name = "";
            for part in parts {
                if part != "" {
                    var_name = part;
                }
            }
            // if is_in_function {
            if !var_is_used(var_name, &file.content) {
                println!("var not used");
                report = Some(Range {
                    start: Position {
                        line: line_index,
                        character: index_equal.unwrap() - 1,
                    },
                    end: Position {
                        line: line_index,
                        character: index_equal.unwrap() - 1 + var_name.len(),
                    },
                });
            } else {
                //(dans un contract)
                println!("var used")
                // logique pour le contrat
            }
        }
        line_index += 1;
    });

    // détecter une variable dans une fonction:
    // je parcours la fonction line par line
    // quand je vois '=' à gauche je sais que c'est une variable (peut être ne pas compter le ':type')
    // j'ajoute cette variable dans une liste
    // si la variable apparait qu'une une fois dans la lsite
    // alors la var n'est pas utilisée

    // pour un contrat juste je check tout le contrat et pas seulement
    // une fonction

    // quand je vois une fonction je regarde si cette variable est utilisé
    // cas : variable utilisé dans une fonction
    // cas : variable utilisé dans un contract

    report
}

fn var_is_used(var_name: &str, file: &str) -> bool {
    let mut nb_occurence = 0;
    file.lines().for_each(|line| {
        let name_with_whitespace = " ".to_owned() + var_name + " ";
        let name_with_comma = " ".to_owned() + var_name + ";";
        let var_is_found =
            line.find(&name_with_whitespace).is_some() || line.find(&name_with_comma).is_some();
        if var_is_found {
            nb_occurence += 1;
        }
        if nb_occurence > 1 {
            return;
        }
    });
    if nb_occurence > 1 {
        return true;
    }
    false
}

impl UnusedVar {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = UnusedVar { _data: data };
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
