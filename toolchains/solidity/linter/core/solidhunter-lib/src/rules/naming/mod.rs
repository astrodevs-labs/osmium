use crate::rules::naming::const_name_snakecase::ConstNameSnakeCase;
use crate::rules::naming::contract_name_pascalcase::ContractNamePascalCase;
use crate::rules::naming::event_name_camelcase::EventNameCamelCase;
use crate::rules::naming::func_name_camelcase::FuncNameCamelCase;
use crate::rules::naming::func_param_name_camelcase::FuncParamNameCamelcase;
use crate::rules::naming::func_visibility::FuncVisibility;
use crate::rules::naming::modifier_name_mixedcase::ModifierNameMixedcase;
use crate::rules::naming::named_parameters_mapping::NamedParametersMapping;
use crate::rules::naming::use_forbidden_name::UseForbiddenName;
use crate::rules::naming::var_name_mixedcase::VarNameMixedCase;
use crate::rules::naming::foundry_func_name::FoundryFuncName;
use crate::rules::types::{RuleEntry, RulesMap};
use crate::rules::RuleBuilder;
use std::collections::HashMap;
use crate::rules::naming::private_vars_leading_underscore::PrivateVarsLeadingUnderscore;

#[macro_use]
pub(crate) mod func_param_name_camelcase;
pub(crate) mod const_name_snakecase;
pub(crate) mod contract_name_pascalcase;
pub(crate) mod event_name_camelcase;
pub(crate) mod func_name_camelcase;
pub(crate) mod func_visibility;
pub(crate) mod modifier_name_mixedcase;
pub(crate) mod named_parameters_mapping;
pub(crate) mod use_forbidden_name;
pub(crate) mod var_name_mixedcase;
pub(crate) mod foundry_func_name;
pub(crate) mod private_vars_leading_underscore;

// List all rules

pub fn create_default_rules() -> Vec<RuleEntry> {
    vec![
        ContractNamePascalCase::create_default(),
        FuncNameCamelCase::create_default(),
        FuncParamNameCamelcase::create_default(),
        UseForbiddenName::create_default(),
        FuncVisibility::create_default(),
        EventNameCamelCase::create_default(),
        ConstNameSnakeCase::create_default(),
        PrivateVarsLeadingUnderscore::create_default(),
        VarNameMixedCase::create_default(),
        ModifierNameMixedcase::create_default(),
        NamedParametersMapping::create_default(),
        FoundryFuncName::create_default(),
    ]
}

pub fn create_rules() -> RulesMap {
    let mut rules: HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert(
        contract_name_pascalcase::RULE_ID.to_string(),
        ContractNamePascalCase::create,
    );
    rules.insert(
        named_parameters_mapping::RULE_ID.to_string(),
        NamedParametersMapping::create,
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
    rules.insert(func_visibility::RULE_ID.to_string(), FuncVisibility::create);
    rules.insert(
        event_name_camelcase::RULE_ID.to_string(),
        EventNameCamelCase::create,
    );
    rules.insert(
        const_name_snakecase::RULE_ID.to_string(),
        ConstNameSnakeCase::create,
    );
    rules.insert(
        private_vars_leading_underscore::RULE_ID.to_string(),
        PrivateVarsLeadingUnderscore::create,
    );
    rules.insert(
        var_name_mixedcase::RULE_ID.to_string(),
        VarNameMixedCase::create,
    );
    rules.insert(
        modifier_name_mixedcase::RULE_ID.to_string(),
        ModifierNameMixedcase::create,
    );
    rules.insert(
        foundry_func_name::RULE_ID.to_string(),
        FoundryFuncName::create,
    );

    rules
}
