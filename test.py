from darn_it import Chunker

with open("README.md", "r") as f:
    readme = f.read()

print(Chunker().get_chunks(readme, 500, "tokens"))