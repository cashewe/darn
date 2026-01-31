# darn-it
(`darn` - *Welsh*, meaning 'piece' or more favourably, 'chunk')

Darn is a rust-backed tool for producing mathematically optimised 'chunks' from markdown-formatted string data. Due to its simple interface and mathematical accuracy, darn is recommended for use by teams looking to move quickly past the problem of chunking at scale and towards more interesting engineering feats - it is likely that specific chunking operations based on context will outpreform it for teams who have the time and desire to manually produce them.

## Setup

To use darn, you must first install it, ideally into a python virtual environment:

```
pip install uv

uv venv --python 3.13
source .venv/bin/activate # or .venv\Scripts\activate on windows

uv pip install darn_it
```

from here, you can import it into your python session for use:

```
from darn_it import Chunker

chunker = Chunker()

with open("file.md", "r") as f:
    text = f.read()

chunker.get_chunks(text=text, chunk_size=500)
```

and darn it will output a list of `Chunk` objects, which include the text of the chunk, along with the start and end index of the chunk for further inspection.

for users who would prefer to use tokens over characters to perform the chunking (i.e. chunks will respect token boundaries, and your chunk_size will reflect max tokens not characters), you can use the `granularity` argument and set it to the string "tokens" i.e.

```
chunker.get_chunks(text=text, chunk_size=500, granularity="tokens")
```
sorry its not an Enum in python, maybe someday :(...

## The Maths Behind the Magic

Darns 'mathematically optimal chunks' are computed by recontextualising the problem of text chunking to be a bounded shortest path problem on a vector of punishment costs, and using the classic dynamic programming algorithm to solve this problem.

a brief summary of the steps involved to achieve this are as follows:

1. Produce a representation of the relevant structures in the text
2. Apply 'rules' with associated 'punishments' for breaking them to each structure
3. create a vector of length = length of input text, whose values are the total 'punishment' for choosign to cut at each character
4. run the dynamic programming algorithm over the vector with a maximum chunk size to calculate the optimal cuts

more information on each of these steps follows.

### 1. Produce a representation of the relevant structures in the text

Darn makes use of the [rust markdown](https://github.com/wooorm/markdown-rs) crate to produce an [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) of the markdown text. This AST is transformed into an '[EnumMap](https://crates.io/crates/enum-map)' object. the EnumMap has keys equal to the unique values of the Enum its built off, and its values are vectors of start and end positions for the structure type. for example:

```
# Hi *Earth*!
```
produces the following MDAST (accoding to the [mardown docs](https://docs.rs/markdown/latest/markdown/fn.to_mdast.html))

```
Root { children: [Heading { children: [Text { value: "Hi ", position: Some(1:3-1:6 (2-5)) }, Emphasis { children: [Text { value: "Earth", position: Some(1:7-1:12 (6-11)) }], position: Some(1:6-1:13 (5-12)) }, Text { value: "!", position: Some(1:13-1:14 (12-13)) }], position: Some(1:1-1:14 (0-13)), depth: 1 }], position: Some(1:1-1:14 (0-13)) }
```

which would create the following EnumMap

```
Heading [0, 13]
Text [0, 13]
Emphasis [5, 12]
```

Importantly, notice that text covers only a single range, despite the fact that there are multiple 'text' nodes in the MDAST. this is because they overlap directly, so are trimmed out to avoid unneccessary computation in future steps. similarly, exactly adjacent nodes of the same time are merged together. This is done to avoid double punishing the same index for a single violation.

### 2. Apply 'rules' with associated 'punishments' for breaking them to each structure

A `Rule` object contains:

- a Node on which it runs (this maps to a value of our EnumMap)
- an 'on_punishment'
- an 'off_punishment'
- (a plain english name, used solely for debugging)

the on / off punishment allows us to tune rules to punish (or reward) a decision to split inside or outside a node of a certain type, for instance if you wanted to encourage splitting between paragraphs you may have a rule like:

```
Rule(
  name="split between paragraphs,
  on_punishment=50,
  off_punishment=0,
  node=Node::Paragraph
)
```

You can have multiple rules for the same node type if prefered.
The reasn for including off punishments are two fold:

1. you may wish to actually reward splitting off a certain node i.e. give a negative punishment for splitting between paragraphs
2. there may be situations where you want a non-linear punishment between values, for instance you may decide you never want to split on titles, but that youd prefer to include at least some text after the title. to do this, you may provide a decreasing punishment function to the 'off_punishment' value, with a static (high) punishment provided to the 'on_punishment'

### 3. create a vector of length = length of input text, whose values are the total 'punishment' for choosign to cut at each character

next, the rules are applied. each index will be punished according to the defined rules based on if it is on or off nodes of every type with a defined rule.
at this point we may choose to reduce this vector to consist only of the final elements of each character. this mapping can be reversed after calculation. by performing the mappings here, we will ensure the following algorithm respects the token boundaries and that the chunk_size will be measured in tokens instead.

**NOTE** the tokeniser currently only supports `cl100k`.

### 4. run the dynamic programming algorithm over the vector with a maximum chunk size to calculate the optimal cuts

Here, we just do a bounded shortest path problem. the jist is:

input: 
- punishment (vector)
- n (maximum chunk size)

instantiate:
- cost (length = punishment vector, initially all values oo)
- cheapest node (length = punishment vector, initially all values are None)
- chunk indicies (unknown length)

- start at the end of the punsihment vector and work backwards to the start
- for each index i:
    - if i + n is greater than the length of the punishment vector
        - set cost[i] = punishment[i]
    - else, find j s.t. i+1 <= j <= i+n, cost[j] is minimum in range
        - cost[i] = cost[j] + punishment [i]
        - cheapest node[i] = j
- starting at index 0, look for 'i' - cheapest element in cost vector within range n
    - chunk indices += i
    - move to index = cheapest node[i]
- repeat prior step until optimal chunk indices is produced.

E.G.

inputs:

current costs = [5, 1, 3, 2, 4, 6], n = 2 (can jump 1 or 2 positions)

initialise:

costs = [∞, ∞, ∞, ∞, 4, 6], cheapest nodes = [NaN, NaN, NaN, NaN, NaN, NaN]

steps:

1.    costs = [∞, ∞, ∞, 6, 4, 6], cheapest nodes = [NaN, NaN, NaN, 5, NaN, NaN]
2.    costs = [∞, ∞, 7, 6, 4, 6], cheapest nodes = [NaN, NaN, 5, 5, NaN, NaN]
3.    costs = [∞, 7, 7, 6, 4, 6], cheapest nodes = [NaN, 4, 5, 5, NaN, NaN]
4.    costs = [12, 7, 7, 6, 4, 6], cheapest nodes = [3, 4, 5, 5, NaN, NaN]
5.    cheapest option up to '2' from point 0 is index 2, so cuts are [0, 2]
6.    index 2s next cheapest is index 4, so cuts are [0, 2, 4]
7.    index 4s next cheapest is index 5, so cuts are [0, 2, 4, 5]
8.    index 5 can hop out of the list, so the final solution is proven to be [0, 2, 4, 5]

### 5. split text

Finally, we perform the splitting on the optimised boundaries, and return to the user the perfect chunks.