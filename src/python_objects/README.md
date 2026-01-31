# Python Objects

In this directory, we include all the rust backed python objects, built using pyO3. On paper, these arent going to be including any real *logic* as such, instead focusing on a clean user interface to the underlying maths. In practice I can sometimes get sloppy so we'll have to see tbh.

the jist of the interface I'd like is something like:

```
@dataclass
class Chunk:
    text: str
    start_index: int
    end_index: int
    #... might be more fields to add in future etc...


class DarnChunker
    get_chunks(text: str, chunk_size: int) -> Iterator[Chunk]:
        # run the rust code here,
        # but output an iterator of the wrapped python objects
```

from a quick google though, iterators are apaz quite hard to build in PyO3 so maybe we will jsut return the list. In my mind at least, we may want to add things like 'section_title', 'table_column_names' etc... as extra fields to the chunk to keep some extra context there for people who may want to add it.