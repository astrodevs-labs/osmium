use crate::rules::naming::contract_name_pascalcase::ContractNamePascalCase;
use crate::rules::naming::func_name_camelcase::FuncNameCamelCase;

/*
use crate::rules::naming::func_param_name_camelcase::FuncParamNameCamelcase;
use crate::rules::naming::use_forbidden_name::UseForbiddenName;
*/
use crate::rules::types::{RuleEntry, RuleType};
use crate::rules::RuleBuilder;
use std::collections::HashMap;

#[macro_use]
pub(crate) mod func_param_name_camelcase;
pub(crate) mod contract_name_pascalcase;
pub(crate) mod func_name_camelcase;
pub(crate) mod use_forbidden_name;

// List all rules

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        ContractNamePascalCase::create_default(),
        FuncNameCamelCase::create_default(),
        /*
        FuncParamNameCamelcase::create_default(),
        UseForbiddenName::create_default(),
        */
    ]
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(
        contract_name_pascalcase::RULE_ID.to_string(),
        ContractNamePascalCase::create,
    );
    rules.insert(
        func_name_camelcase::RULE_ID.to_string(),
        FuncNameCamelCase::create,
    );
    /*
    rules.insert(
        "func-param-name-camelcase".to_string(),
        FuncParamNameCamelcase::create,
    );
    rules.insert(
        UseForbiddenName::RULE_ID.to_string(),
        UseForbiddenName::create,
    );
    */

    rules
}
