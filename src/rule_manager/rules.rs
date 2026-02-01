use crate::md_parser::NodeType;
use crate::rule_manager::punishments::{
    const_punishment, inverse_triangular_punishment, reverse_linear_punishment, PunishmentFn,
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
        on_punishment: const_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Paragraph,
    },
    Rule {
        name: "Prefer cutting paragraphs in the center if neccessary.",
        on_punishment: inverse_triangular_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Paragraph,
    },
    Rule {
        name: "Maintain some context after titles, and dont cut them",
        on_punishment: const_punishment::<100>,
        off_punishment: reverse_linear_punishment,
        node_type: NodeType::Heading,
    },
    Rule {
        name: "Dont cut blockquotes",
        on_punishment: const_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Blockquote,
    },
    Rule {
        name: "Dont cut code blocks",
        on_punishment: const_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Code,
    },
    Rule {
        name: "Dont cut words",
        on_punishment: const_punishment::<150>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Word,
    },
    Rule {
        name: "Dont cut sentences",
        on_punishment: const_punishment::<100>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Sentence,
    },
    Rule {
        name: "Dont cut tables",
        on_punishment: const_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::Table,
    },
    Rule {
        name: "Dont cut table rows",
        on_punishment: const_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::TableRow,
    },
    Rule {
        name: "Dont cut table cells",
        on_punishment: const_punishment::<100>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::TableCell,
    },
    Rule {
        name: "Dont cut lists",
        on_punishment: const_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::List,
    },
    Rule {
        name: "Dont cut list items",
        on_punishment: const_punishment::<100>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::ListItem,
    },
    Rule {
        name: "Prefer cutting lists in the center if neccessary.",
        on_punishment: inverse_triangular_punishment::<50>,
        off_punishment: const_punishment::<0>,
        node_type: NodeType::List,
    }
];

