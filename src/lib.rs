//! Maximal matching on undirected graphs.
//!
//! Value-exact port of `networkx.algorithms.matching.maximal_matching`.
//! Edge iteration order replicates `nx.Graph.edges()`: nodes in insertion
//! order; for each node u, neighbours in insertion order; (u, v) emitted
//! only when u has not yet appeared as a neighbour of an earlier node
//! (i.e. the first time u is seen as the "source").

/// A compact adjacency representation whose edge-iteration order mirrors
/// `nx.Graph` — insertion-order nodes, insertion-order neighbours.
pub struct Graph {
    /// Node labels in first-appearance order.
    node_order: Vec<String>,
    /// For each node index: neighbour indices in insertion order (deduped).
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            node_order: Vec::new(),
            adj: Vec::new(),
        }
    }

    fn intern(&mut self, label: &str) -> usize {
        if let Some(pos) = self.node_order.iter().position(|n| n == label) {
            return pos;
        }
        let idx = self.node_order.len();
        self.node_order.push(label.to_owned());
        self.adj.push(Vec::new());
        idx
    }

    /// Add an undirected edge (parallel edges deduplicated like `nx.Graph`).
    pub fn add_edge(&mut self, u: &str, v: &str) {
        if u == v {
            // nx.Graph stores self-loops; maximal_matching skips them (a
            // self-loop would use one node twice). The node still registers.
            let ui = self.intern(u);
            if !self.adj[ui].contains(&ui) {
                self.adj[ui].push(ui);
            }
            return;
        }
        let ui = self.intern(u);
        let vi = self.intern(v);
        if !self.adj[ui].contains(&vi) {
            self.adj[ui].push(vi);
        }
        if !self.adj[vi].contains(&ui) {
            self.adj[vi].push(ui);
        }
    }

    /// Iterate edges exactly as `nx.Graph.edges()` does:
    ///
    /// For each node u (in insertion order), for each neighbour v of u
    /// (in insertion order), emit (u, v) only if u has not yet been
    /// "exhausted" — i.e. no earlier node has u in its adjacency list
    /// from the perspective of the already-seen set.  Concretely: emit
    /// (u, v) iff v's index was not yet marked "seen as lhs" at that point.
    ///
    /// NetworkX's EdgeView skips (u,v) when `v` was already yielded as an
    /// lhs (i.e. `v in seen`), because the reverse (v, u) was already
    /// emitted.  `seen` grows node by node.
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let n = self.node_order.len();
        let mut seen = vec![false; n];
        let mut out = Vec::new();
        for u in 0..n {
            for &v in &self.adj[u] {
                if !seen[v] {
                    out.push((u, v));
                }
            }
            seen[u] = true;
        }
        out.into_iter()
    }

    /// Greedy maximal matching in `nx.Graph.edges()` order.
    ///
    /// Returns matched pairs as node-index pairs.  The caller canonicalises
    /// and sorts for output.  Self-loops (u == v) are skipped exactly as
    /// networkx does — a self-loop is not a valid matching edge.
    pub fn maximal_matching(&self) -> Vec<(usize, usize)> {
        let n = self.node_order.len();
        let mut matched = vec![false; n];
        let mut result = Vec::new();
        for (u, v) in self.edges() {
            if u != v && !matched[u] && !matched[v] {
                matched[u] = true;
                matched[v] = true;
                result.push((u, v));
            }
        }
        result
    }

    pub fn node_label(&self, idx: usize) -> &str {
        &self.node_order[idx]
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse an edge-list text (stdin format: `u v` per line, `#` comments and
/// blank lines skipped).  Returns the `Graph` with insertion order matching
/// `nx.Graph().add_edges_from(lines)`.
pub fn parse_edgelist(text: &str) -> Graph {
    let mut g = Graph::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_ascii_whitespace();
        let u = match parts.next() {
            Some(s) => s,
            None => continue,
        };
        let v = match parts.next() {
            Some(s) => s,
            None => continue,
        };
        g.add_edge(u, v);
    }
    g
}

/// Canonicalise a matched edge for output: lexicographically smaller label
/// first (same tie-breaking networkx uses when it stringifies sets — but we
/// don't rely on nx set order; we sort the output ourselves).
pub fn canonical_edge<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str) {
    if a <= b { (a, b) } else { (b, a) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_debug_assert() {
        use clap::CommandFactory;
        crate::cli::Cli::command().debug_assert();
    }

    #[test]
    fn edge_iteration_path() {
        // Path 1-2-3-4-5 inserted in order: G.edges() = [(1,2),(2,3),(3,4),(4,5)]
        let mut g = Graph::new();
        for (u, v) in [("1", "2"), ("2", "3"), ("3", "4"), ("4", "5")] {
            g.add_edge(u, v);
        }
        let edges: Vec<_> = g.edges().collect();
        assert_eq!(edges, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn edge_iteration_out_of_order() {
        // Insertion order 3-4, 1-2, 2-3, 4-5 → nodes [3,4,1,2,5]
        // G.edges() = [(3,4),(3,2),(4,5),(1,2)]
        let mut g = Graph::new();
        for (u, v) in [("3", "4"), ("1", "2"), ("2", "3"), ("4", "5")] {
            g.add_edge(u, v);
        }
        // node 0=3, 1=4, 2=1, 3=2, 4=5
        let edges: Vec<_> = g.edges().collect();
        // u=0(3): adj=[1(4),3(2)] → (0,1),(0,3); mark 0 seen
        // u=1(4): adj=[0(3),4(5)] → 0 seen skip; (1,4); mark 1 seen
        // u=2(1): adj=[3(2)] → (2,3); mark 2 seen
        // u=3(2): adj=[2(1),0(3)] → 2 seen skip, 0 seen skip; mark 3 seen
        // u=4(5): adj=[1(4)] → 1 seen skip; mark 4 seen
        assert_eq!(edges, vec![(0, 1), (0, 3), (1, 4), (2, 3)]);
    }

    #[test]
    fn matching_path() {
        let mut g = Graph::new();
        for (u, v) in [("1", "2"), ("2", "3"), ("3", "4"), ("4", "5")] {
            g.add_edge(u, v);
        }
        let m = g.maximal_matching();
        let canon: Vec<_> = m
            .iter()
            .map(|&(u, v)| canonical_edge(g.node_label(u), g.node_label(v)))
            .collect();
        // nx: {(1,2),(3,4)}
        assert!(canon.contains(&("1", "2")));
        assert!(canon.contains(&("3", "4")));
        assert_eq!(canon.len(), 2);
    }

    #[test]
    fn matching_star() {
        let mut g = Graph::new();
        for (u, v) in [("1", "2"), ("1", "3"), ("1", "4"), ("1", "5")] {
            g.add_edge(u, v);
        }
        let m = g.maximal_matching();
        assert_eq!(m.len(), 1);
        let (u, v) = m[0];
        let (a, b) = canonical_edge(g.node_label(u), g.node_label(v));
        // nx: {(1,2)}
        assert_eq!((a, b), ("1", "2"));
    }
}

// Re-export cli for the debug_assert test above
pub mod cli;
