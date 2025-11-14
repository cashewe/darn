# darn-it (welsh, meaning 'piece' or more favourably, 'chunk')
opinionated text chunker that respects markdown format. Built in rust for speed / safety, but hooked in to python via maturin or otherwise to allow Data Scientists to use it for text processing.

the tool will receive a *markdown string* (i.e. anything that isn't valid markdown will not be guaranteed to work, dont especially care if someone tries anyways though) and output 'chunks' - an exhaustive collection of all the text in the original string but split up according to rules.

the tool will receive:
- some input string
- a chunk size 
- a definition of size (this can be used to account for token based chunking vs character based chunking etc...)
- an option to include overlap between chunks of a set size
- an option to allow chunks to be larger than the given size in the event that certain 'high priority' rules would be violated otherwise (for instance, if i want to strictly never split a table up, I may need to allow a table chunk to be larger than the set size.

and will output:
- some collection of 'chunks'
  - start character index
  - content
  - length? (i.e. in characters / tokens depending on the strategy selected)
we *may* prefer this as a generator rather than a list output?

the act of chunking will be performed in a way that aims to respect parts of text (English language only). It will do this with a hierarchical list of 'rules' it will attempt to chunk by - and we may want to expose these to the user to allow them to reorder or turn certain rules off etc...

## for examples of rules:
- if we have two paragraphs, and the first is much smaller than the estimated chunk size, we will still return it as a single chunk if the following paragraph is small enough to also be a chunk of its own. if the following paragraph will be split either way, we'd be ok to carry part of it into the current chunk.
- do not split a sentence between chunks unless absolutely neccessary
- keep entire bullet point / numbered / alphabetised lists in one chunk where possible
- keep sub lists in one chunk if the above isn't possible
- keep entire tables in one chunk where possible
- keep entire code snippets where possible
- keep full quotes where possible (i.e. everything between two quotation marks, or everything between two sets of three quotation marks to cover longer strings)
- keep proper nouns etc... where possible
- include context where possible (i.e. the last sentence before a structure such as a quote, list, table, etc... will be included in the chunk with the structure)

## the tool will need to be designed to the following spec:
- scale to add new opinionated rules easily, and rearrange them etc... without issue
- allow for some rules to dynamically edit the underlying text, without loosing the correct start index of the chunk from the original text (for instance, if we physically *cant* keep a whole table together in one chunk, we may get around this by refusing to split rows of the table between chunks, and duplicating the tables column headers into the second chunk that contains it. this should not mean the index becomes inaccurate.)
- lowest possible memory cost
- have some means of identifying each type of 'structure' in a string accurately. (we may want to allow users to access this anyways so it becomes like a context-ful string that can also perform chunking like context_string.paragraphs(), context_string.tables(), context_string.chunk(), etc...)

## other ideas i have that might be good to explore (i.e. the design ideally wouldn't lock us out of any of these without warning)
- the rules might be able to be turned on or off, or rearranged in terms of importance in init?
- there might be categories for strict vs flexible rules, where a strict rule is willing to overshoot the chunksize in order to be obeyed whereas a flexible rule is a preference but it will be broken to maintain chunk size. in either case, the violation should be minimised. behaviour may differ based on these rules (for instance as mentioned above, if tables are set as flexible, the column headers may exist in multiple chunks). users may be able to set the rule category themselves if they want.
- a definition of the size of a violation might be allowed per rule - for instance a sentence may aim to have as little as possible be cut off between chunks, whilst in the case of a bullet pointed list you may aim to have as even a split as possible? 

## how do we ensure later rules dont end up accidentally violating old ones? 
i guess each rule will need to yield one of two lists:
- acceptable chunk indicies
- unacceptable chunk indicies
these indicies may need to be spans i.e. 'you cannot chunk within this span' vs 'you can chunk at this specific index'. we'd then have something come and pick where to put the chunk start / end points based on these values, along with the chunk size?

## steps to build:
1. create AST of input markdown string (this is valuable anyways, i haven't seen anyone do it exhaustively. theres got to be a reason for that mind you... -> assume non trivial)
2. create chunk generator that yields chunks based on a list of start / end indexes its provided
3. create rule system that scales to 'N' rules, allows users to set new rule priority, flexibility etc..., which uses the AST as its framework for applying rules.
4. create boundry resolver that uses the rules suggested start / end indicies and decides where to actually perform the splitting


AST for markdown is well defined in the 'mdast' specification:
https://github.com/syntax-tree/mdast
which luckily provides us with all the unit tests etc... we'll need.

to complete step one ill need to learn alot more about AST's - but i think thatll be interesting tbh. 
https://github.com/wooorm/markdown-rs/blob/main/src/to_mdast.rs
^^ this crate has a markdown AST already included so on paper dont need to build my own. id still rather hand spin it so that i can learn more about the process though. I can use the above as a reference throughout, but the final solution should be handmade i.e. no swiping prebuilt solutions and no 'vibe coding' some crap tier slop.

to start with then, we could probably actully get away with starting with the chunk generator - and immediately get something pip installable. the markdown AST is the real value add here, but we can have a default chunk set up and running very quickly i think.