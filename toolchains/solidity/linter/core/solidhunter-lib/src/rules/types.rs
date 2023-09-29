use crate::linter::SolidFile;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    fn diagnose(&self, file: &SolidFile, files: &[SolidFile]) -> Vec<LintDiag>;
}

pub type RulesMap = HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>;
