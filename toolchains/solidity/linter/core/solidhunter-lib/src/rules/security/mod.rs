use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

#[macro_use]
pub(crate) mod no_inline_assembly;

// List all rules
use crate::rules::security::no_inline_assembly::NoInlineAssembly;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![NoInlineAssembly::create_default()]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(
        no_inline_assembly::RULE_ID.to_string(),
        NoInlineAssembly::create,
    );

    rules
}
