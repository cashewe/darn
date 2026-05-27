# Analyser

The analyser was a user requested object (by popular demmand of two whole users - i.e. probably 100% of them...) which can allow users to see and understand the underlying punishment functions making up the decision boundaries.

the most likely implementation is as follows:

- instead of burning the punishments, pass them to the user
- the user can then call the 'analyse' method with a chunk number to analyse the punishments of that and the surrounding chunks
- i recon a stacked line graph showing the total punishment at each index, along with the split of what contributed to that punishment would be handy. unfortunately as of right now, we update a single punishment vector, so the cost breakdown is slightly harder. we would need to preserve the costs per rule to have this work... i dont think that would be crippling though. tbh the punishments are applied per rule, not per structure; so it may be possible to do this in a sick manner?