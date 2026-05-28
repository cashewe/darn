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
print(chunks.punishment_breakdown)