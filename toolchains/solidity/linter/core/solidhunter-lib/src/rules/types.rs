use crate::linter::SolidFile;
use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleEntry {
    pub id: String,
    pub severity: Severity,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub name: String,
    pub includes: Vec<String>,
    pub plugins: Vec<String>,
    pub rules: Vec<RuleEntry>,
}

pub trait RuleType: Send + Sync + 'static {
    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag>;
}
