// to parse the ast we get from the package
// will need to:
//   - pull the current node type
//   - ull the start / end index of the entire node (i.e. incl. all children) and push as a 'tuple' to the hashmap
//     the example suggests these indexes are provided by the crate i.e.:
//   ```Root { children: [Heading { children: [Text { value: "Hi ", position: Some(1:3-1:6 (2-5)) }, Emphasis { children: [Text { value: "Earth", position: Some(1:7-1:12 (6-11)) }], position: Some(1:6-1:13 (5-12)) }, Text { value: "!", position: Some(1:13-1:14 (12-13)) }], position: Some(1:1-1:14 (0-13)), depth: 1 }], position: Some(1:1-1:14 (0-13)) }```
// 
// this can then be used in any rules to identify the relevant text to operate on
//
// the parser may well be useful on its own to expose in the package - so get it right you coward.
// may also want functions to draw out info from other places such as:
//  - the index of the start of the sentence prior to a given index
//  - the indeces of all sentence ending punctuation within an index range
//  - maybe a third thing


pub struct MdastParser {
    
}