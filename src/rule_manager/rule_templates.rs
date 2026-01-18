/// trait to represent anything capable of splitting markdown text up.
pub trait MdSplitter {
    fn split_md(&self) -> &str;  // this spec is likely to change as i work, its just here for now to ensure it can compile
}

// here is where we'd put our splitters, if we had any -_-