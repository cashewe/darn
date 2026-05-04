use crate::md_parser::NodeType;
use crate::rule_manager::punishments::{
    const_punishment, inverse_triangular_punishment, reverse_linear_punishment, PunishmentFn,
};

/// a rule defines the cost function on a given node
#[derive(Clone)]
pub struct Rule {
    pub on_punishment: PunishmentFn,
    pub on_scale: usize,
    pub off_punishment: PunishmentFn,
    pub off_scale: usize,
    pub node_type: NodeType,
}

/// this will be the default rules list, users can and maybe are encouraged
/// to write their own replacements as it is, quite frankly, asssss
pub static RULES: &[Rule] = &[
    Rule { // Dont cut paragraphs.
        on_punishment: const_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Paragraph,
    },
    Rule { //Prefer cutting paragraphs in the center if neccessary.
        on_punishment: inverse_triangular_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Paragraph,
    },
    Rule { // Maintain some context after titles, and dont cut them.
        on_punishment: const_punishment,
        on_scale: 100,
        off_punishment: reverse_linear_punishment,
        off_scale: 0,
        node_type: NodeType::Heading,
    },
    Rule { // Dont cut blockquotes.
        on_punishment: const_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Blockquote,
    },
    Rule { // Dont cut code blocks.
        on_punishment: const_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Code,
    },
    Rule { // Dont cut words.
        on_punishment: const_punishment,
        on_scale: 150,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Word,
    },
    Rule { // Dont cut sentences.
        on_punishment: const_punishment,
        on_scale: 100,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Sentence,
    },
    Rule { // Dont cut tables.
        on_punishment: const_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::Table,
    },
    Rule { // Dont cut table rows.
        on_punishment: const_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::TableRow,
    },
    Rule { // Dont cut table cells.
        on_punishment: const_punishment,
        on_scale: 100,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::TableCell,
    },
    Rule { // Dont cut lists.
        on_punishment: const_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::List,
    },
    Rule { // Dont cut list items.
        on_punishment: const_punishment,
        on_scale: 100,
        off_punishment: const_punishment,
        off_scale: 0,
        node_type: NodeType::ListItem,
    },
    Rule { // Prefer cutting lists in the center if neccessary.
        on_punishment: inverse_triangular_punishment,
        on_scale: 50,
        off_punishment: const_punishment,
        off_scale:0,
        node_type: NodeType::List,
    }
];

