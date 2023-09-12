use crate::rules::types::{RuleEntry, RuleType};
use std::collections::HashMap;

#[macro_use]
pub mod quotes;

// List all rules

use crate::rules::miscellaneous::quotes::Quotes;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![Quotes::create_default()]
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert("quotes".to_string(), Quotes::create);

    rules
}
