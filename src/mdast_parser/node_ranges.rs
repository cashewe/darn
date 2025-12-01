// in this file we will:
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
    /// do we assume start is before end or should i enforce that here?
    pub fn add(&mut self, kind: NodeType, start: u32, end: u32) {
        self.ranges[kind].push(
            NodeStartEnd(start, end)
        )
    }

    /// get the vect for a given node type
    /// as a borrowed iterable rather than rawdogging the accessor
    /// optionally bound the iterator with a given start / end period
    pub fn get_ranges(
        &self,
        kind: NodeType,
        start_at: Option<u32>,
        end_at: Option<u32>,
    ) -> impl Iterator<Item = &NodeStartEnd> {
        let ranges = &self.ranges[kind];

        // begin iterating only after start is no longer less than the selected threshold
        let start_idx = if let Some(start_threshold) = start_at {
            ranges.partition_point(|r| r.start < start_threshold)
        } else {
            0
        };

        ranges[start_idx..]
            .iter()
            .take_while(move |r| {
                if let Some(end_threshold) = end_at {
                    r.end <= threshold  // break out once we're too begin
                } else {
                    true
                }
            })
    }

    /// merge adjacent or overlapping ranges of a shared type
    /// this will stop us over calculating later
    /// does it belong here or somewhere else?
    /// when should it be called to minimise the API width?
    pub fn deduplicate_ranges() {}
}