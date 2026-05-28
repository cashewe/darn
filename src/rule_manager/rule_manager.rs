use crate::md_parser::{NodeType, NodeStartEnd, NodeRanges};
use crate::rule_manager::{RULES, Rule};


/// the output vector of punishments
#[derive(Clone)]
pub struct PunishmentVector {
    pub totals:   Vec<usize>,
    pub per_rule: Vec<Vec<usize>>,
}

pub struct RuleManager;

impl RuleManager {
    /// linear rule executor
    pub fn build_punishment_vector(
        node_ranges: &NodeRanges,
        total_len: usize,
        rules: Option<&[Rule]>,
    ) -> PunishmentVector {
        let rules = rules.unwrap_or(&RULES);
        let mut per_rule = vec![vec![0usize; total_len]; rules.len()];
        
        for (node_type, ranges) in node_ranges.ranges.iter() {
            for (i, rule) in rules.iter().enumerate() {
                if rule.node_type != node_type { continue; }
                Self::_apply_rule(rule, ranges, total_len, &mut per_rule[i]);
            }
        }

        let totals = per_rule.iter().fold(vec![0usize; total_len], |mut acc, v| {
            acc.iter_mut().zip(v).for_each(|(a, x)| *a += x);
            acc
        });

        PunishmentVector { totals, per_rule }
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
                    rule.off_scale,
                    cursor,
                    start,
                    output,
                );
            }

            // within a node of a given type, apply on_punishment
            if start < end {
                Self::_apply_segment(
                    rule.on_punishment,
                    rule.on_scale,
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
                rule.off_scale,
                cursor,
                total_len,
                output,
            );
        }
    }

    /// apply the punishment to the range described
    #[inline]
    fn _apply_segment(
        f: fn(usize, usize, &mut [usize]),
        scale: usize,
        start: usize,
        end: usize,
        output: &mut [usize],
    ) {
        let len = end - start;
        let mut tmp = vec![0usize; len];

        f(scale, len, &mut tmp);

        for (dst, val) in output[start..end].iter_mut().zip(tmp) {
            *dst += val;
        }
    }
}