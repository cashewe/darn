# The Jist, as it is...

so general approach to the problem atm is:

- parse string to mdast, using prewritten package as this is incredibly complex work (https://docs.rs/markdown/latest/markdown/fn.to_mdast.html)
- for each rule (thou shalt not split mid way into a table...) write a 'rule' which outputs a list of indexes where you can or maybe cant split, lest you incur a penalty
- with the set of all these index lists, run an optimisation alg to find the best points to split on to minimise penalty costs
- return the split text to the user

the folder structure reflects this modular nature:

1. first is the ast parsing (swiped)
2. second is the rule application
3. third is the algorithms

this is likely over complciated vs the ideal, but gives me a chance to do the following cool shit:

- write a python package in rust
- do some OR and actual maths for once
- write complex code rather than really useless simple things
- actually hand spin some algs rather than xgboost.fitting
- hopefully build something with controlled expansion, rather than an unweildy mess of custom paths etc... for every edge case we come to
- fit into an ecosystem of standard tools (md formatted text for instance) rather than some very specific niche

overall i think it should be fun and stay interesting due to the combination of programmatic and mathematical challenges its likely to expose me to, though im not going in with any delusions about the overall value to the world about the resultant solution.