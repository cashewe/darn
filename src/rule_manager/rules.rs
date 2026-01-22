use md_parser::NodeType;
use crate::rule_manager::rule_templates::{
    fifty_punishment, inverse_triangular_punishment, zero_punishment,
};

/// a rule defines the cost function on a given node
struct Rule {
    name: &'static str,
    on_punishment: fn(i32) -> Vec<i32>,
    off_punishment: fn(i32) -> Vec<i32>,
    node_type: NodeType,
}

// rules assembled
// we actually want to define these in the registry i think
// then give it a 'get_rules(node_type: NodeType) -> Vec<Rule>' function

let dont_cut_paragraphs = Rule {
    name: "Dont cut paragraphs",
    on_punishment: fifty_punishment,
    off_punishment: zero_punishment,
    node_type: NodeType::Paragraph,
};

let prefer_cut_paragraphs_center = Rule {
    name: "Prefer cutting paragraphs in the center if neccessary.",
    on_punishment: inverse_triangular_punishment,
    off_punishment: zero_punishment,
    node_type: NodeType::Paragraph,
};

let dont_cut_title = Rule {
    name: "Dont cut titles",
    on_punishment: fifty_punishment,
    off_punishment: zero_punishment,
    node_type: NodeType::Title,
};

let dont_cut_quote = Rule {
    name: "Dont cut blockquotes",
    on_punishment: fifty_punishment,
    off_punishment: zero_punishment,
    node_type: NodeType::Blockquote,
};

let dont_cut_code = Rule {
    name: "Dont cut code blocks",
    on_punishment: fifty_punishment,
    off_punishment: zero_punishment,
    node_type: NodeType::Code,
};

