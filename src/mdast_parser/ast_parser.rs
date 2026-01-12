// to parse the ast we get from the package
// will need to:
//   - pull the current node type
//   - ull the start / end index of the entire node (i.e. incl. all children) and push as a 'tuple' to the hashmap
//     the example suggests these indexes are provided by the crate i.e.:
//   ```Root { children: [Heading { children: [Text { value: "Hi ", position: Some(1:3-1:6 (2-5)) }, Emphasis { children: [Text { value: "Earth", position: Some(1:7-1:12 (6-11)) }], position: Some(1:6-1:13 (5-12)) }, Text { value: "!", position: Some(1:13-1:14 (12-13)) }], position: Some(1:1-1:14 (0-13)), depth: 1 }], position: Some(1:1-1:14 (0-13)) }```
// 
// this can then be used in any rules to identify the relevant text to operate on
use crate::mdast_parser::{NodeRanges, NodeType};
use markdown::mdast::Node;
use markdown::mdast::Node::*;
use markdown::{to_mdast, ParseOptions};

pub struct MdParser;

impl MdParser {
    /// turn a markdown structured string into a collection of node start / end indices
    pub fn parse(markdown_string: &str) -> NodeRanges{
        let root = to_mdast(markdown_string, &ParseOptions::default())?;
        let mut ranges = NodeRanges::new();

        for child in &root.children {
            Self::visit_node(child, ranges);
        }

        ranges.deduplicate_ranges();
        ranges
    }

    /// recursion for extracting start and end from all nodes in the mdast
    fn visit_node(node: &Node, ranges: &mut NodeRanges) {
        if let Some((kind, start, end)) = Self::extract_position(node) {
            ranges.add(kind, start, end)
        }

        // recurisvely re-enter this function for all children of the current node...
        // how do i hit the children?
        // some but not all node types have a 'children' trait
        // so i will probably have to match for each of those types manually and then enter again for them...
        // cant be assed this evening - this will be a tomorrow task i think.
    }

    /// get type, start and end position from a given node object
    fn extract_position(node: &Node) -> Option<(NodeType, u32, u32)> {
        // I might be missing something here buuuut
        // since markdown doesnt include like a 'Positioned' trait
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