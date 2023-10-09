use crate::rules::types::{RuleEntry, RulesMap};
use std::collections::HashMap;

#[macro_use]
pub mod line_maxlen;
pub mod custom_errors;
pub mod empty_block;
pub mod function_max_lines;
pub mod global_import;
pub mod max_states_count;
pub mod reason_string;
pub mod one_contract_per_file;

// List all rules

use crate::rules::best_practises::custom_errors::CustomErrors;
use crate::rules::best_practises::empty_block::EmptyBlock;
use crate::rules::best_practises::function_max_lines::FunctionMaxLines;
use crate::rules::best_practises::global_import::GlobalImport;
use crate::rules::best_practises::line_maxlen::LineMaxLen;
use crate::rules::best_practises::max_states_count::MaxStatesCount;
use crate::rules::best_practises::one_contract_per_file::OneContractPerFile;
use crate::rules::best_practises::reason_string::ReasonString;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        LineMaxLen::create_default(),
        MaxStatesCount::create_default(),
        FunctionMaxLines::create_default(),
        ReasonString::create_default(),
        OneContractPerFile::create_default(),
        CustomErrors::create_default(),
        GlobalImport::create_default(),
        EmptyBlock::create_default(),
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
    rules.insert(custom_errors::RULE_ID.to_string(), CustomErrors::create);
    rules.insert(reason_string::RULE_ID.to_string(), ReasonString::create);
    rules.insert(
        one_contract_per_file::RULE_ID.to_string(),
        OneContractPerFile::create,
    );
    rules.insert(global_import::RULE_ID.to_string(), GlobalImport::create);
    rules.insert(empty_block::RULE_ID.to_string(), EmptyBlock::create);

    rules
}
