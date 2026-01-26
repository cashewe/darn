use crate::md_parser::{NodeType, NodeStartEnd, NodeRanges};
use crate::rule_manager::{RULES, Rule};

pub struct RuleManager;

impl RuleManager {
    /// linear rule executor
    pub fn build_punishment_vector(
        node_ranges: &NodeRanges,
        total_len: usize,
    ) -> Vec<usize> {

        let mut result = vec![0usize; total_len];

        for (node_type, ranges) in node_ranges.ranges.iter() { // parallelise this later?

            let rules = Self::_get_rules_for_node_type(node_type);
            if rules.is_empty() {
                continue;
            }

            for rule in rules {
                Self::_apply_rule(
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
        rule: &Rule,
        ranges: &Vec<NodeStartEnd>,
        total_len: usize,
        output: &mut [usize],
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
        f: fn(usize, &mut [usize]),
        start: usize,
        end: usize,
        output: &mut [usize],
    ) {
        let len = end - start;
        let mut tmp = vec![0usize; len];

        f(len, &mut tmp);

        for (dst, val) in output[start..end].iter_mut().zip(tmp) {
            *dst += val;
        }
    }

    /// filter registered rules to those of a give type
    fn _get_rules_for_node_type(node_type: NodeType) -> Vec<&'static Rule> {
        RULES.iter()
            .filter(|rule| rule.node_type == node_type)
            .collect()
    }
}