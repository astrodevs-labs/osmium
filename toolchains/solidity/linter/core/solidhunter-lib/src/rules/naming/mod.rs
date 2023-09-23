use crate::rules::naming::contract_name_pascalcase::ContractNamePascalCase;
use crate::rules::naming::func_name_camelcase::FuncNameCamelCase;
use crate::rules::naming::const_name_snakecase::ConstNameSnakeCase;

use crate::rules::naming::func_param_name_camelcase::FuncParamNameCamelcase;
use crate::rules::naming::use_forbidden_name::UseForbiddenName;
use crate::rules::types::{RuleEntry, RulesMap};
use crate::rules::RuleBuilder;
use std::collections::HashMap;

#[macro_use]
pub(crate) mod func_param_name_camelcase;
pub(crate) mod contract_name_pascalcase;
pub(crate) mod func_name_camelcase;
pub(crate) mod use_forbidden_name;
pub(crate) mod const_name_snakecase;

// List all rules

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        ContractNamePascalCase::create_default(),
        FuncNameCamelCase::create_default(),
        FuncParamNameCamelcase::create_default(),
        UseForbiddenName::create_default(),
        ConstNameSnakeCase::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(
        contract_name_pascalcase::RULE_ID.to_string(),
        ContractNamePascalCase::create,
    );
    rules.insert(
        func_name_camelcase::RULE_ID.to_string(),
        FuncNameCamelCase::create,
    );
    rules.insert(
        func_param_name_camelcase::RULE_ID.to_string(),
        FuncParamNameCamelcase::create,
    );
    rules.insert(
        use_forbidden_name::RULE_ID.to_string(),
        UseForbiddenName::create,
    );
    rules.insert(
        const_name_snakecase::RULE_ID.to_string(),
        ConstNameSnakeCase::create
    );

    rules
}
