use clap::Parser;

/// Maximal matching on an undirected graph.
///
/// Reads an edge list from stdin (`u v` per line; `#` comments and blank
/// lines are skipped; string node labels; parallel edges deduplicated).
/// Outputs one matched edge per line as `u\tv`, canonicalised (smaller
/// label first) and sorted lexicographically.
///
/// The matching produced is value-exact with `networkx.maximal_matching`
/// (BSD-3-Clause): edges are iterated in networkx's `G.edges()` order
/// (node insertion order × neighbour insertion order), and the same greedy
/// rule applies.
#[derive(Parser, Debug)]
#[command(name = "rsomics-maximal-matching", version)]
pub struct Cli {
    /// Emit output as a JSON array of [u, v] pairs instead of TSV.
    #[arg(long)]
    pub json: bool,
}
