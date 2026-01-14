use enum_map::Enum;
use markdown::mdast::Node;
use markdown::mdast::Node::*;


/// All MDAST Nodes minus the extra slop i.e. 'text', 'volume', etc...
#[derive(Debug, Copy, Clone, Enum, PartialEq, Eq, Hash)]
pub enum NodeType {
    Root,
    Blockquote,
    FootnoteDefinition,
    MdxJsxFlowElement,
    List,
    MdxjsEsm,
    Toml,
    Yaml,
    Break,
    InlineCode,
    InlineMath,
    Delete,
    Emphasis,
    MdxTextExpression,
    FootnoteReference,
    Html,
    Image,
    ImageReference,
    MdxJsxTextElement,
    Link,
    LinkReference,
    Strong,
    Text,
    Code,
    Math,
    MdxFlowExpression,
    Heading,
    Table,
    ThematicBreak,
    TableRow,
    TableCell,
    ListItem,
    Definition,
    Paragraph,
}
