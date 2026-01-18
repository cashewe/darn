use serde::{Deserialize, Serialize};
use std::fmt;

use markdown::mdast::Node;

use crate::md_parsing::MdSplitter;

/// structure representing a splitting rule for MDASTs
/// each Rule will provide a penalty for violating a certain behaviour on some MDAST structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    name: String,
    description: String,
    structure_type: Node, // https://docs.rs/markdown/latest/markdown/mdast/enum.Node.html an md data structure -> i hope this works...
    penalty: u16, // only relative size matters, so a cheaper format that still has scope for a big 'infinity' esque cost works
    rule_template: Box<dyn MdSplitter>,
}

impl Rule {
    /// reset the penalty value
    pub fn set_penalty(&mut self, p: u32) {
        self.penalty = p;
    }

    // need to expose an 'execute' method once ive got a template written up
}

impl fmt::Display for Rule {
    /// this needs some more thought, but for now lets just show the user some info on the rule
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "**{}**:\n\npenalty: {}\n*{}*", self.name, self.penalty, self.description,)
    }
}

// tests
// test penalty can be set
// test user cannot edit the various fields directly? does this matter?