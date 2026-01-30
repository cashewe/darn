# ChunkOptimiser

Here, we just do a bounded shortest path problem. the jist is:

input: 
- punishment (vector)
- n (maximum chunk size)

instantiate:
- cost (length = punishment vector, initially all values oo)
- cheapest node (length = punishment vector, initially all values are None)
- chunk indicies (unknown length)

- start at the end of the punsihment vector and work backwards to the start
- for each index i:
    - if i + n is greater than the length of the punishment vector
        - set cost[i] = punishment[i]
    - else, find j s.t. i+1 <= j <= i+n, cost[j] is minimum in range
        - cost[i] = cost[j] + punishment [i]
        - cheapest node[i] = j
- starting at index 0, look for 'i' - cheapest element in cost vector within range n
    - chunk indices += i
    - move to index = cheapest node[i]
- repeat prior step until optimal chunk indices is produced.

## example

inputs:

current costs = [5, 1, 3, 2, 4, 6], n = 2 (can jump 1 or 2 positions)

initialise:

costs = [∞, ∞, ∞, ∞, 4, 6], cheapest nodes = [NaN, NaN, NaN, NaN, NaN, NaN]

steps:

1.    costs = [∞, ∞, ∞, 6, 4, 6], cheapest nodes = [NaN, NaN, NaN, 5, NaN, NaN]
2.    costs = [∞, ∞, 7, 6, 4, 6], cheapest nodes = [NaN, NaN, 5, 5, NaN, NaN]
3.    costs = [∞, 7, 7, 6, 4, 6], cheapest nodes = [NaN, 4, 5, 5, NaN, NaN]
4.    costs = [12, 7, 7, 6, 4, 6], cheapest nodes = [3, 4, 5, 5, NaN, NaN]
5.    cheapest option up to '2' from point 0 is index 2, so cuts are [0, 2]
6.    index 2s next cheapest is index 4, so cuts are [0, 2, 4]
7.    index 4s next cheapest is index 5, so cuts are [0, 2, 4, 5]
8.    index 5 can hop out of the list, so the final solution is proven to be [0, 2, 4, 5]
