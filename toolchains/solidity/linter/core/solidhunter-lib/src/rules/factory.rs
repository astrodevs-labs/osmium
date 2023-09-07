use std::collections::HashMap;
use crate::rules::types::*;
use crate::rules::create_rules;

pub struct RuleFactory {
    _buildables: HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>,
    _rules : Vec<Box<dyn RuleType>>,
}

impl Default for RuleFactory {
    fn default() -> Self {
        RuleFactory {
            _buildables: create_rules(),
            _rules: Vec::new(),
        }
    }
}

impl RuleFactory {
    pub fn new() -> RuleFactory {
        RuleFactory {
            _buildables: HashMap::new(),
            _rules: Vec::new(),
        }
    }

    pub fn create_rule(&self, rule: RuleEntry) -> Box<dyn RuleType>
    {
        let rule_type = self._buildables.get(&rule.id);
        if rule_type.is_none() {
            panic!("Rule {} not found", &rule.id);
        }
        rule_type.unwrap()(rule)
    }
}