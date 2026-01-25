// rule_registry:
//  - remove rules from registry - will need some identifiers for rules i guess
//  - run rules of a given node type async
//  - publically run for all nodes in a tree

use md_parser::NodeType;
use crate::rule_manager::{RULES, Rule};

pub struct RuleRegistry {
    rules: Vec<&'static Rule>,
}

impl RuleRegistry {
    fn new() -> Self {
        Self {
            rules: RULES.iter().collect(),
        }
    }

    /// filter registered rules to those of a give type
    fn _get_rules_for_node_type(&self, node_type: NodeType) -> Vec<&'static Rule> {
        self.rules.iter()
            .filter(|rule| rule.node_type == node_type)
            .collect()
    }
}