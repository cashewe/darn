from darn_it import (
    Chunker,
    NodeType,
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
        nodetype=NodeType.Sentence
    )
]
print(Chunker(rules).get_chunks(readme, 500, "tokens", overlap=50))