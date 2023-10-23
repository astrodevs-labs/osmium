use crate::linter::SolidFile;
use crate::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleEntry {
    pub id: String,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub name: String,
    pub rules: Vec<RuleEntry>,
}

pub trait RuleType: Send + Sync + 'static {
    fn diagnose(&self, file: &SolidFile, files: &[SolidFile]) -> Vec<LintDiag>;
}

pub type RulesMap = HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>;
