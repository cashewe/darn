// in this file we will:
// define a means of pushing new node ranges to the struct
// define a means of merging adjacent ranges
use enum_map::EnumMap;
use crate::mdast_parser::NodeType;

/// the start and end of the range as identified in the algo
pub struct NodeStartEnd {
    start: u32,  // are these big enough? how do we handle the ambiguity?????
    end: u32,
}

/// HashMap Enum hybrid thing i found for representing
/// every range for a given NodeType
pub struct NodeRanges {
    pub ranges: EnumMap<NodeType, Vec<NodeStartEnd>>,
}

impl NodeRanges {
    /// creator for default Mapping
    pub fn new() -> Self {
        Self {
            ranges: EnumMap::default(),
        }
    }

    /// add range to the heap
    pub fn add() {}

    /// merge adjacent or overlapping ranges of a shared type
    pub fn merge_ranges() {}
}