# ChunkOptimiser

this component will run optimisation algs over the punishment vector in order to optimise where we cut.

things we need to account for:
- chunk size limits (strict / flexible?)
- run time
- minimum chunk size also needs to be encouraged right? or itll just shoot for like as many tiny chunks as possible...

treating this as a shortest path problem, dynamic programming is the correct way to solve it. unfortunately thats alot less fun than building silly algorithms, so i may just goof off and do those too, call it science and ask for a pay rise for the pleasure...

## architecture

we need something that stores and selects which algorithm to run?
we need some data object for storing chunking indexes? -> or should it actually perform the chunking tbh...

## potential algorithms

- forwardsgreedy: *take the cheapest option within the given chunk size, iteratively*
- backwardsgreedy: *take the cheapest option within the given chunk size, starting from the end, iteratively
- genetic: *breed based on cheapest option out of two parent options each time?*
- genetic: *breed with alternating parets?*
- dynamic programming: *this is effectively a shortest path problem right?*

going in, id expect dynamic programming to yield the best solution - so we should probs aim to get it in first.