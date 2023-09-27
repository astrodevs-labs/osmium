use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

#[macro_use]
pub mod line_maxlen;
pub mod function_max_lines;
pub mod max_states_count;
pub mod reason_string;
pub mod unused_var;

// List all rules

use crate::rules::best_practises::function_max_lines::FunctionMaxLines;
use crate::rules::best_practises::line_maxlen::LineMaxLen;
use crate::rules::best_practises::max_states_count::MaxStatesCount;
use crate::rules::best_practises::reason_string::ReasonString;
use crate::rules::best_practises::unused_var::UnusedVar;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        LineMaxLen::create_default(),
        MaxStatesCount::create_default(),
        FunctionMaxLines::create_default(),
        ReasonString::create_default(),
        UnusedVar::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(line_maxlen::RULE_ID.to_string(), LineMaxLen::create);

    rules.insert(
        max_states_count::RULE_ID.to_string(),
        MaxStatesCount::create,
    );
    rules.insert(
        function_max_lines::RULE_ID.to_string(),
        FunctionMaxLines::create,
    );
    rules.insert(reason_string::RULE_ID.to_string(), ReasonString::create);
    rules.insert(unused_var::RULE_ID.to_string(), UnusedVar::create);

    rules
}
