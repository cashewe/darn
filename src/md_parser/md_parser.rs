use crate::md_parser::{NodeRanges, NodeType};
use markdown::mdast::Node;
use markdown::mdast::Node::*;
use markdown::{to_mdast, ParseOptions};

pub struct MdParser;

impl MdParser {
    /// turn a markdown structured string into a collection of node start / end indices
    pub fn parse(markdown_string: &str) -> NodeRanges{
        let root = to_mdast(markdown_string, &ParseOptions::default()).unwrap();
        let mut ranges = NodeRanges::new();

        // not all Nodes have children - prove to the compiler this one does as its a root
        if let Node::Root(root_node) = root {
            for child in &root_node.children {
                Self::visit_node(child, &mut ranges);
            }
        }

        ranges.deduplicate_ranges();
        ranges
    }

    /// recursion for extracting start and end from all nodes in the mdast
    fn visit_node(node: &Node, ranges: &mut NodeRanges) {
        if let Some((kind, start, end)) = Self::extract_position(node) {
            ranges.add(kind, start, end)
        }

        // not all node types have children, so we filter here
        // would be good to find a cleaner way to do this
        match node {
            Node::Root(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Blockquote(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::FootnoteDefinition(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::MdxJsxFlowElement(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::List(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Delete(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Emphasis(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::MdxJsxTextElement(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Link(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::LinkReference(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Strong(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Heading(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Table(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::TableRow(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::TableCell(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::ListItem(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            Node::Paragraph(n) => {
                for child in &n.children {
                    Self::visit_node(child, ranges);
                }
            }
            _ => {} // Leaf nodes without children
        }
    }

    /// get type, start and end position from a given node object
    fn extract_position(node: &Node) -> Option<(NodeType, u32, u32)> {
        // I might be missing something here buuuut
        // since markdown doesnt include like a 'Positioned' trait
        // and i need to map beat by beat
        // i think i litterally have no choice but to extract position manually like this?
        let (kind, position) = match node {
            Root(n) => (NodeType::Root, &n.position),
            Blockquote(n) => (NodeType::Blockquote, &n.position),
            FootnoteDefinition(n) => (NodeType::FootnoteDefinition, &n.position),
            MdxJsxFlowElement(n) => (NodeType::MdxJsxFlowElement, &n.position),
            List(n) => (NodeType::List, &n.position),
            MdxjsEsm(n) => (NodeType::MdxjsEsm, &n.position),
            Toml(n) => (NodeType::Toml, &n.position),
            Yaml(n) => (NodeType::Yaml, &n.position),
            Break(n) => (NodeType::Break, &n.position),
            InlineCode(n) => (NodeType::InlineCode, &n.position),
            InlineMath(n) => (NodeType::InlineMath, &n.position),
            Delete(n) => (NodeType::Delete, &n.position),
            Emphasis(n) => (NodeType::Emphasis, &n.position),
            MdxTextExpression(n) => (NodeType::MdxTextExpression, &n.position),
            FootnoteReference(n) => (NodeType::FootnoteReference, &n.position),
            Html(n) => (NodeType::Html, &n.position),
            Image(n) => (NodeType::Image, &n.position),
            ImageReference(n) => (NodeType::ImageReference, &n.position),
            MdxJsxTextElement(n) => (NodeType::MdxJsxTextElement, &n.position),
            Link(n) => (NodeType::Link, &n.position),
            LinkReference(n) => (NodeType::LinkReference, &n.position),
            Strong(n) => (NodeType::Strong, &n.position),
            Text(n) => (NodeType::Text, &n.position),
            Code(n) => (NodeType::Code, &n.position),
            Math(n) => (NodeType::Math, &n.position),
            MdxFlowExpression(n) => (NodeType::MdxFlowExpression, &n.position),
            Heading(n) => (NodeType::Heading, &n.position),
            Table(n) => (NodeType::Table, &n.position),
            ThematicBreak(n) => (NodeType::ThematicBreak, &n.position),
            TableRow(n) => (NodeType::TableRow, &n.position),
            TableCell(n) => (NodeType::TableCell, &n.position),
            ListItem(n) => (NodeType::ListItem, &n.position),
            Definition(n) => (NodeType::Definition, &n.position),
            Paragraph(n) => (NodeType::Paragraph, &n.position),
            _ => return None,  // took this from SO -> might prefer to raise here tbh...
        };

        position.as_ref().map(|pos| {
            (
                kind,
                pos.start.offset as u32,
                pos.end.offset as u32,
            )
        })
    }
}