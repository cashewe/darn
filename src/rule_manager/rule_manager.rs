use md_parser::{NodeType, NodeStartEnd, NodeRanges};
use crate::rule_manager::{RULES, Rule};

pub struct RuleManager {
    rules: Vec<&'static Rule>, // do we need to store this here or can we just import it raw?
}

impl RuleManager {
    fn new() -> Self {
        Self {
            rules: RULES.iter().collect(),
        }
    }

    /// linear rule executor
    pub fn build_punishment_vector(
        &self,
        node_ranges: &NodeRanges,
        total_len: usize,
    ) -> Vec<i32> {

        let mut result = vec![0i32; total_len];

        for (node_type, ranges) in node_ranges.ranges.iter() { // parallelise this later?

            let rules = self._get_rules_for_node_type(*node_type);
            if rules.is_empty() {
                continue;
            }

            for rule in rules {
                self._apply_rule(
                    rule,
                    ranges,
                    total_len,
                    &mut result,
                );
            }
        }

        result
    }

    /// apply the rule to the given ranges for the nodetype.
    fn _apply_rule(
        &self,
        rule: &Rule,
        ranges: &Vec<NodeStartEnd>,
        total_len: usize,
        output: &mut [i32],
    ) {

        let mut cursor = 0usize;

        for r in ranges {

            let start = r.start as usize;
            let end   = r.end   as usize;

            // outside of a node of a given type, apply off_punishment
            if cursor < start {
                Self::_apply_segment(
                    rule.off_punishment,
                    cursor,
                    start,
                    output,
                );
            }

            // within a node of a given type, apply on_punishment
            if start < end {
                Self::_apply_segment(
                    rule.on_punishment,
                    start,
                    end,
                    output,
                );
            }

            cursor = end;
        }

        // after ranges end there may be further indexes to apply the off punishment to
        if cursor < total_len {
            Self::_apply_segment(
                rule.off_punishment,
                cursor,
                total_len,
                output,
            );
        }
    }

    /// apply the punishment to the range described
    #[inline]
    fn _apply_segment(
        f: fn(usize, &mut [i32]),
        start: usize,
        end: usize,
        output: &mut [i32],
    ) {
        let len = end - start;
        let mut tmp = vec![0i32; len];

        f(len, &mut tmp);

        for (dst, val) in output[start..end].iter_mut().zip(tmp) {
            *dst += val;
        }
    }

    /// filter registered rules to those of a give type
    fn _get_rules_for_node_type(&self, node_type: NodeType) -> Vec<&'static Rule> {
        self.rules.iter()
            .filter(|rule| rule.node_type == node_type)
            .collect()
    }
}