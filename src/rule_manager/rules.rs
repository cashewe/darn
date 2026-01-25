use md_parser::NodeType;
use crate::rule_manager::rule_templates::{
    fifty_punishment, inverse_triangular_punishment, zero_punishment, PunishmentFn,
};

/// a rule defines the cost function on a given node
struct Rule {
    name: &'static str,
    on_punishment: PunishmentFn,
    off_punishment: PunishmentFn,
    node_type: NodeType,
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
        node_type: NodeType::Title,
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

