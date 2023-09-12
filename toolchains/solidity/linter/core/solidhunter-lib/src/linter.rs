use crate::error::SolidHunterError;
use crate::rules::factory::RuleFactory;
use crate::rules::rule_impl::{create_rules_file, parse_rules};
use crate::rules::types::*;
use crate::types::*;
use std::fs;

use glob::glob;
use solc_wrapper::{Solc, SourceUnit};

pub struct SolidFile {
    pub data: SourceUnit,
    pub path: String,
    pub content: String,
}

pub struct SolidLinter {
    files: Vec<SolidFile>,
    rule_factory: RuleFactory,
    rules: Vec<Box<dyn RuleType>>,
}

impl Default for SolidLinter {
    fn default() -> Self {
        let linter = SolidLinter::new(&String::new());
        linter
    }
}

impl SolidLinter {
    pub fn new(rules_config: &String) -> Self {
        let mut linter = SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::default(),
            rules: Vec::new(),
        };
        linter._create_rules(&rules_config, true);
        return linter;
    }

    fn _create_rules(&mut self, rules_config: &String, first: bool) {
        let res = parse_rules(rules_config.as_str());
        match res {
            Ok(rules) => {
                for rule in rules.rules {
                    self.rules.push(self.rule_factory.create_rule(rule));
                }
            }
            Err(_) => {
                create_rules_file(rules_config.as_str());
                if first {
                    self._create_rules(rules_config, false);
                }
            }
        }
    }

    fn _file_exists(&self, path: &str) -> bool {
        for file in &self.files {
            if file.path == path {
                return true;
            }
        }
        false
    }

    fn _update_file_ast(&mut self, path: &str, ast: SourceUnit) {
        for file in &mut self.files {
            if file.path == path {
                file.data = ast.clone();
            }
        }
    }

    fn _add_file(&mut self, path: &str, ast: SourceUnit, content: &str) {
        let file = SolidFile {
            data: ast,
            path: String::from(path),
            content: String::from(content),
        };
        self.files.push(file);
    }

    pub fn parse_file(&mut self, filepath: String) -> LintResult {
        let res = Solc::default().extract_ast_file(filepath.clone());

        if res.is_err() {
            println!("{:?}", res);
            return Err(SolidHunterError::SolcError(res.unwrap_err()));
        }
        if self._file_exists(filepath.as_str()) {
            self._update_file_ast(filepath.as_str(), res.expect("ast not found"));
        } else {
            let content = fs::read_to_string(filepath.clone())
                .map_err(|e| SolidHunterError::IoError(e.to_string()))?;
            self._add_file(
                filepath.as_str(),
                res.expect("ast not found"),
                content.as_str(),
            );
        }
        let mut res: Vec<LintDiag> = Vec::new();

        for rule in &self.rules {
            let mut diags = rule.diagnose(&self.files[0], &self.files);
            res.append(&mut diags);
        }
        Ok(res)
    }

    pub fn parse_content(&mut self, filepath: String, content: &String) -> LintResult {
        let res = Solc::default().extract_ast_content(content.to_string());

        if res.is_err() {
            println!("{:?}", res);
            return Err(SolidHunterError::SolcError(res.unwrap_err()));
        }

        if self._file_exists(filepath.as_str()) {
            self._update_file_ast(filepath.as_str(), res.expect("ast not found"));
        } else {
            self._add_file(
                filepath.as_str(),
                res.expect("ast not found"),
                content.as_str(),
            );
        }

        let mut res: Vec<LintDiag> = Vec::new();

        for rule in &self.rules {
            let mut diags = rule.diagnose(&self.files[0], &self.files);
            res.append(&mut diags);
        }
        Ok(res)
    }

    pub fn parse_folder(&mut self, folder: String) -> Vec<LintResult> {
        let mut result: Vec<LintResult> = Vec::new();
        if let Ok(entries) = glob(&*(folder + "/**/*.sol")) {
            for entry in entries {
                if let Ok(path) = entry {
                    result.push(
                        self.parse_file(String::from(path.into_os_string().into_string().unwrap())),
                    );
                }
            }
        }
        result
    }

    pub fn delete_file(&mut self, path: String) {
        loop {
            let idx = self.files.iter().position(|x| x.path == path);
            match idx {
                Some(idx) => {
                    self.files.remove(idx);
                }
                None => {
                    break;
                }
            }
        }
    }
}
