# darn-it
(`darn` - *Welsh*, 'därn', meaning 'piece' or more favourably, 'chunk')

Darn is a python tool for producing optimal 'chunks' from markdown-formatted string data. The tool makes use of a novel 'punishment' based approach to determining chunk boundaries, which in practice can be seen to be an improvement over traditional "semantic" chunking methods. To avoid adding unexpected behaviours (hidden text cleaning etc...), `darn` assumes [ASCII](https://www.ascii-code.com/) compliance. pre-cleaning to remove non-ASCII compliant characters should be performed by users of the package ahead of time to avoid errors in processing.

for more on the method that underpins `darn`, see the accompanying [blog post](https://cashewe.github.io/blog/optimal-chunking-strats/). for detailed examples, see the [documentation]().

## Setup

To use `darn`, you must first install it, ideally into a python virtual environment:

```
pip install uv

uv venv --python 3.13
source .venv/bin/activate # or .venv\Scripts\activate on windows

uv pip install darn_it
```

from here, you can import it into your python session for use:

```
from darn_it import (
    Chunker,
    PyNodeType,
)

chunker = Chunker(
    [
        Rule(
            on_punishment="const",
            on_scale=10,
            off_punishment="const",
            off_scale=0,
            nodetype=PyNodeType.Sentence
        ),  # ... add custom rules here. if none are provided, darn will fall back on its curated list of defaults
    ]
)

with open("file.md", "r") as f:
    text = f.read()

chunker.get_chunks(
    text=text, 
    chunk_size=500,
    # granularity="tokens", # either "tokens" or "characters"
    # model="gpt-4o", # only consumed if `granularity` is set to `tokens`
    # overlap=50, # respects `granularity` variable
)
```

Rules can be applied to any structure from the [mdast](https://github.com/syntax-tree/mdast) specification, along with the `Sentence` and `Word`.
A full list of currently supported punishment functions for rules is given below:

| punishment | meaning |
|------------|---------|
| const | a static punishment value |
| linear | a punishment value that increments by 1 per character, starting at the provided input |
| reverse_linear | a punishment that decrements by 1 per character, starting at the provided input and stopping at 0 |
| triangular | a punishment which peaks at the provided value in the middle of the structure, starting and ending at 0 |
| inverse_triangular | a punishment which peaks at the start and end of the structure at the provided value, and hits 0 in the middle |

