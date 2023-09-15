use crate::rules::types::{RuleEntry, RuleType};
use std::collections::HashMap;

#[macro_use]
pub(crate) mod import_on_top;

// List all rules
use crate::rules::order::import_on_top::ImportOnTop;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![ImportOnTop::create_default()]
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(import_on_top::RULE_ID.to_string(), ImportOnTop::create);

    rules
}
