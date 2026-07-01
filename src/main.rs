use clap::Parser;
use rsomics_maximal_matching::cli::Cli;
use rsomics_maximal_matching::{canonical_edge, parse_edgelist};
use std::io::{self, Read};

fn main() {
    let args = Cli::parse();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let graph = parse_edgelist(&input);
    let matching = graph.maximal_matching();

    // Collect canonical edges and sort for deterministic output.
    let mut edges: Vec<(&str, &str)> = matching
        .iter()
        .map(|&(u, v)| canonical_edge(graph.node_label(u), graph.node_label(v)))
        .collect();
    edges.sort_unstable();

    if args.json {
        let arr: Vec<[&str; 2]> = edges.iter().map(|&(u, v)| [u, v]).collect();
        println!(
            "{}",
            serde_json::to_string(&arr).expect("json serialization")
        );
    } else {
        for (u, v) in &edges {
            println!("{u}\t{v}");
        }
    }
}
