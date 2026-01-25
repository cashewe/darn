# RuleManager
This module contains the code which handles the application of 'rules' onto an EnumMap object for node ranges created in MdParser.

The basic jist is that we want to create a vector, whose length is the same as the length of the input text, which describes how willing we are to perform a chunking split on any given index. This is done by use of additive 'punishments' - integer costs asociated with breaking some simple splitting rules. this vector will then be put through some optimisation algorithm which will aim to create chunks of no more than a given size whilst minimising the incurred costs.

A 'Rule' is some simple English statement such as 'you should not split within a title'. to allow for 'prefer X, but fall back to Y if not' type logic, rules are additive - meaning youre punishment for splitting on a given index will be the sum cost of all punsihments associated with that index. this way you can:

- discourage splitting in a node with a blanket punishment
- control where in the node the algorithm would prefer to split if forced by including some non-static punishment on top of this (i.e. a function which peaks in the middle of the node)

In a move sure to confuse the bois and girlz alike, we split our punishments into 'on_punishment' and 'off_punishment' functions. the 'on_punishment' is applied within nodes of a given type whilst the 'off_punishment' is applied outside of these nodes. whilst it is likely that the majority of 'off_punsihments' will be the static value 0, there may be situations in which you want to encourage splitting based on a node type but not within it - as an example, you may say:

- never split part way into a title (translates to a constant relatively high on_punishment for NodeType::Title)
- prefer not to split 'near after' a title to maintain at least some context for the section (translates to a decreasing off_punishment for NodeType::Title)

This may be over complicating matters, but ill be damned if that isnt sort of my thing by this point.