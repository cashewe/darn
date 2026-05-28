from darn_it import (
    Chunker,
    PyNodeType,
    Rule,
)

with open("README.md", "r") as f:
    readme = f.read()

print(Chunker().get_chunks(readme, 500, "tokens", overlap=50))

rules = [
    Rule(
        on_punishment="const",
        on_scale=10,
        off_punishment="const",
        off_scale=0,
        nodetype=PyNodeType.Sentence
    )
]

chunks = Chunker(rules).get_chunks(readme, 100, "tokens", overlap=50)
print(chunks.chunks)

import tempfile, webbrowser

svg = chunks.analyse(5)

with tempfile.NamedTemporaryFile(suffix=".svg", delete=False, mode="w") as f:
    f.write(svg)
    webbrowser.open(f"file://{f.name}")