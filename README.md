# rsomics-maximal-matching

Maximal matching on undirected graphs — single-binary CLI, value-exact with `networkx.maximal_matching`.

## Usage

```
echo -e "a b\nb c\nc d" | rsomics-maximal-matching
a	b
c	d
```

Reads an edge list from stdin (`u v` per line; `#` comments and blank lines skipped; string node
labels; parallel edges deduplicated like `nx.Graph`).  Outputs one matched edge per line as
`u<TAB>v`, canonicalised (smaller label first lexicographically) and sorted.

Flag `--json` emits a JSON array of `[u, v]` pairs instead.

## Algorithm

Greedy maximal matching in `networkx.Graph.edges()` order.  The output is the same *set* of
matched edges as `networkx.maximal_matching` produces when given the same edge list.

Edge iteration order replicates `nx.Graph`:

- Nodes are enumerated in **first-appearance (insertion) order**.
- For each node `u`, its neighbours are visited in **neighbour insertion order**.
- `(u, v)` is emitted only when `u` has not yet appeared as a neighbour of any earlier node
  (i.e. `u` has not been "seen as lhs"); this is exactly networkx's `EdgeView` skip rule.

The greedy algorithm then scans those edges: for each `(u, v)`, if neither is already matched,
add `(u, v)` to the matching and mark both matched.

Node labels are interned to contiguous `usize` indices so the hot path is a simple `Vec<bool>`
lookup — no hash map inside the loop.

## Origin

This crate is an independent Rust reimplementation of `networkx.algorithms.matching.maximal_matching`
based on:

- The NetworkX 3.x source (`networkx/algorithms/matching.py`, BSD-3-Clause)
- Black-box behaviour testing against NetworkX 3.6.1

NetworkX is MIT/BSD-3-Clause licensed.  Credit: Aric Hagberg, Pieter Swart, and contributors —
<https://networkx.org>.

License: MIT OR Apache-2.0.
