use crate::md_parser::NodeType;
use crate::rule_manager::punishments::{
    fifty_punishment, inverse_triangular_punishment, zero_punishment, reverse_linear_punishment, PunishmentFn,
};

/// a rule defines the cost function on a given node
pub struct Rule {
    pub name: &'static str,
    pub on_punishment: PunishmentFn,
    pub off_punishment: PunishmentFn,
    pub node_type: NodeType,
}

/// rules assembled
/// This will need extending, potentially need to find a more sustainable way to define them...
pub static RULES: &[Rule] = &[
    Rule {
        name: "Dont cut paragraphs",
        on_punishment: fifty_punishment,
        off_punishment: zero_punishment,
        node_type: NodeType::Paragraph,
    },
    Rule {
        name: "Prefer cutting paragraphs in the center if neccessary.",
        on_punishment: inverse_triangular_punishment,
        off_punishment: zero_punishment,
        node_type: NodeType::Paragraph,
    },
    Rule {
        name: "Dont cut titles",
        on_punishment: fifty_punishment,
        off_punishment: zero_punishment,
        node_type: NodeType::Heading,
    },
    Rule {
        name: "Maintain some context after titles",
        on_punishment: fifty_punishment,
        off_punishment: reverse_linear_punishment,
        node_type: NodeType::Heading,
    },
    Rule {
        name: "Dont cut blockquotes",
        on_punishment: fifty_punishment,
        off_punishment: zero_punishment,
        node_type: NodeType::Blockquote,
    },
    Rule {
        name: "Dont cut code blocks",
        on_punishment: fifty_punishment,
        off_punishment: zero_punishment,
        node_type: NodeType::Code,
    }
];

