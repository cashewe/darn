use pyo3::prelude::*;
use crate::md_parser::NodeType;

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyNodeType {
    Root, Blockquote, FootnoteDefinition, MdxJsxFlowElement,
    List, MdxjsEsm, Toml, Yaml, Break, InlineCode, InlineMath,
    Delete, Emphasis, MdxTextExpression, FootnoteReference, Html,
    Image, ImageReference, MdxJsxTextElement, Link, LinkReference,
    Strong, Text, Code, Math, MdxFlowExpression, Heading, Table,
    ThematicBreak, TableRow, TableCell, ListItem, Definition,
    Paragraph, Word, Sentence,
}

impl From<PyNodeType> for NodeType {
    fn from(py: PyNodeType) -> NodeType {
        match py {
            PyNodeType::Root               => NodeType::Root,
            PyNodeType::Blockquote         => NodeType::Blockquote,
            PyNodeType::FootnoteDefinition => NodeType::FootnoteDefinition,
            PyNodeType::MdxJsxFlowElement  => NodeType::MdxJsxFlowElement,
            PyNodeType::List               => NodeType::List,
            PyNodeType::MdxjsEsm           => NodeType::MdxjsEsm,
            PyNodeType::Toml               => NodeType::Toml,
            PyNodeType::Yaml               => NodeType::Yaml,
            PyNodeType::Break              => NodeType::Break,
            PyNodeType::InlineCode         => NodeType::InlineCode,
            PyNodeType::InlineMath         => NodeType::InlineMath,
            PyNodeType::Delete             => NodeType::Delete,
            PyNodeType::Emphasis           => NodeType::Emphasis,
            PyNodeType::MdxTextExpression  => NodeType::MdxTextExpression,
            PyNodeType::FootnoteReference  => NodeType::FootnoteReference,
            PyNodeType::Html               => NodeType::Html,
            PyNodeType::Image              => NodeType::Image,
            PyNodeType::ImageReference     => NodeType::ImageReference,
            PyNodeType::MdxJsxTextElement  => NodeType::MdxJsxTextElement,
            PyNodeType::Link               => NodeType::Link,
            PyNodeType::LinkReference      => NodeType::LinkReference,
            PyNodeType::Strong             => NodeType::Strong,
            PyNodeType::Text               => NodeType::Text,
            PyNodeType::Code               => NodeType::Code,
            PyNodeType::Math               => NodeType::Math,
            PyNodeType::MdxFlowExpression  => NodeType::MdxFlowExpression,
            PyNodeType::Heading            => NodeType::Heading,
            PyNodeType::Table              => NodeType::Table,
            PyNodeType::ThematicBreak      => NodeType::ThematicBreak,
            PyNodeType::TableRow           => NodeType::TableRow,
            PyNodeType::TableCell          => NodeType::TableCell,
            PyNodeType::ListItem           => NodeType::ListItem,
            PyNodeType::Definition         => NodeType::Definition,
            PyNodeType::Paragraph          => NodeType::Paragraph,
            PyNodeType::Word               => NodeType::Word,
            PyNodeType::Sentence           => NodeType::Sentence,
        }
    }
}