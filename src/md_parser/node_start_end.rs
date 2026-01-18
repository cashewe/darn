use std::fmt;

/// the start and end of the range as identified in the algo
#[derive(Clone, Copy)]
pub struct NodeStartEnd {
    pub start: u32,  // are these big enough? how do we handle the ambiguity?????
    pub end: u32,
}

/// allow users to easily view the range
impl fmt::Display for NodeStartEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}