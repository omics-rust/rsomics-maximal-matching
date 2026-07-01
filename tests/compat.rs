//! Value-exactness tests against networkx 3.6.1 maximal_matching.
//!
//! All expected sets are derived from running
//! `networkx.maximal_matching(G)` on the same edge list and hardcoded here.
//! They are NOT derived from this crate's own output.
//!
//! Canonicalisation: each edge is (smaller_label, larger_label) by
//! lexicographic string order.  The full set is then sorted lexicographically
//! before comparison.

use rsomics_maximal_matching::{canonical_edge, parse_edgelist};

fn run(edgelist: &str) -> Vec<(String, String)> {
    let g = parse_edgelist(edgelist);
    let raw = g.maximal_matching();
    let mut v: Vec<(String, String)> = raw
        .into_iter()
        .map(|(u, w)| {
            let (a, b) = canonical_edge(g.node_label(u), g.node_label(w));
            (a.to_owned(), b.to_owned())
        })
        .collect();
    v.sort_unstable();
    v
}

fn edges_to_str(edges: &[(&str, &str)]) -> String {
    edges
        .iter()
        .map(|(u, v)| format!("{u} {v}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn expected(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = pairs
        .iter()
        .map(|&(u, w)| (u.to_owned(), w.to_owned()))
        .collect();
    v.sort_unstable();
    v
}

// --------------------------------------------------------------------------
// Hand-crafted graphs
// --------------------------------------------------------------------------

#[test]
fn path_p5() {
    // P5: 1-2-3-4-5, edges in order
    // nx: {(1,2),(3,4)}
    let got = run("1 2\n2 3\n3 4\n4 5");
    assert_eq!(got, expected(&[("1", "2"), ("3", "4")]));
}

#[test]
fn path_p5_out_of_order() {
    // Same graph, different insertion order → different G.edges() → same matching here
    let got = run("3 4\n1 2\n2 3\n4 5");
    assert_eq!(got, expected(&[("1", "2"), ("3", "4")]));
}

#[test]
fn star_k1_4() {
    // Center 1, leaves 2-5
    // nx: {(1,2)}
    let got = run("1 2\n1 3\n1 4\n1 5");
    assert_eq!(got, expected(&[("1", "2")]));
}

#[test]
fn cycle_c6() {
    // 1-2-3-4-5-6-1
    // nx edges: [(1,2),(1,6),(2,3),(3,4),(4,5),(5,6)]
    // nx matching: {(1,2),(3,4),(5,6)}
    let got = run("1 2\n2 3\n3 4\n4 5\n5 6\n6 1");
    assert_eq!(got, expected(&[("1", "2"), ("3", "4"), ("5", "6")]));
}

#[test]
fn complete_k4() {
    // All 6 edges of K4 in lexicographic order
    // nx matching: {(1,2),(3,4)}
    let got = run("1 2\n1 3\n1 4\n2 3\n2 4\n3 4");
    assert_eq!(got, expected(&[("1", "2"), ("3", "4")]));
}

#[test]
fn bipartite_k2_3() {
    // L0-R0, L0-R1, L0-R2, L1-R0, L1-R1, L1-R2
    // nx nodes: [L0,R0,R1,R2,L1]
    // nx edges: [(L0,R0),(L0,R1),(L0,R2),(R0,L1),(R1,L1),(R2,L1)]
    // nx matching: {(L0,R0),(L1,R1)}
    let got = run("L0 R0\nL0 R1\nL0 R2\nL1 R0\nL1 R1\nL1 R2");
    assert_eq!(got, expected(&[("L0", "R0"), ("L1", "R1")]));
}

#[test]
fn disconnected_with_triangle() {
    // (1,2),(3,4),(5,6),(1,3)
    // nx nodes: [1,2,3,4,5,6]
    // nx edges: [(1,2),(1,3),(3,4),(5,6)]
    // nx matching: {(1,2),(3,4),(5,6)}
    let got = run("1 2\n3 4\n5 6\n1 3");
    assert_eq!(got, expected(&[("1", "2"), ("3", "4"), ("5", "6")]));
}

#[test]
fn single_edge() {
    let got = run("1 2");
    assert_eq!(got, expected(&[("1", "2")]));
}

#[test]
fn two_isolated_edges() {
    let got = run("1 2\n3 4");
    assert_eq!(got, expected(&[("1", "2"), ("3", "4")]));
}

#[test]
fn empty_no_edges() {
    let got = run("");
    assert_eq!(got, Vec::<(String, String)>::new());
}

#[test]
fn comments_and_blank_lines_skipped() {
    let input = "# header\na b\n\n# comment\nb c\nc d\n";
    // Same as "a b\nb c\nc d" → nx matching {(a,b),(c,d)}
    let got = run(input);
    assert_eq!(got, expected(&[("a", "b"), ("c", "d")]));
}

#[test]
fn parallel_edges_deduped() {
    // Adding a b twice must behave like adding once
    let got = run("a b\na b\nb c");
    // G has edges: a-b, b-c  → matching {(a,b)}
    assert_eq!(got, expected(&[("a", "b")]));
}

#[test]
fn string_nodes_with_mixed_labels() {
    // "b c" inserted before "a b"
    // nx nodes: [b,c,a], edges: [(b,c),(b,a)], matching: {(b,c)}
    let got = run("b c\na b");
    assert_eq!(got, expected(&[("b", "c")]));
}

// --------------------------------------------------------------------------
// Random graphs (gnm) — edge lists hardcoded from networkx output
// --------------------------------------------------------------------------

// gnm(20, 40, seed=7) — 8 pairs expected
const GNM_20_40_7_EDGES: &[(&str, &str)] = &[
    ("1", "12"),
    ("1", "18"),
    ("1", "2"),
    ("1", "13"),
    ("1", "7"),
    ("1", "17"),
    ("2", "17"),
    ("2", "7"),
    ("2", "18"),
    ("2", "19"),
    ("2", "14"),
    ("2", "8"),
    ("3", "11"),
    ("3", "18"),
    ("3", "17"),
    ("3", "16"),
    ("4", "10"),
    ("4", "9"),
    ("4", "13"),
    ("5", "17"),
    ("5", "7"),
    ("5", "13"),
    ("6", "16"),
    ("6", "18"),
    ("6", "19"),
    ("7", "18"),
    ("7", "9"),
    ("9", "18"),
    ("9", "16"),
    ("9", "14"),
    ("10", "13"),
    ("10", "15"),
    ("11", "14"),
    ("11", "19"),
    ("12", "18"),
    ("13", "15"),
    ("14", "18"),
    ("15", "17"),
    ("15", "18"),
    ("17", "18"),
];
const GNM_20_40_7_EXPECTED: &[(&str, &str)] = &[
    ("1", "12"),
    ("10", "15"),
    ("13", "4"),
    ("14", "9"),
    ("17", "3"),
    ("18", "2"),
    ("19", "6"),
    ("5", "7"),
];

#[test]
fn gnm_20_40_seed7() {
    let input = edges_to_str(GNM_20_40_7_EDGES);
    let got = run(&input);
    assert_eq!(
        got,
        expected(GNM_20_40_7_EXPECTED),
        "gnm(20,40,seed=7): matching mismatch vs networkx 3.6.1"
    );
}

// gnm(50, 100, seed=42) — 19 pairs expected
const GNM_50_100_42_EDGES: &[(&str, &str)] = &[
    ("0", "17"),
    ("0", "43"),
    ("0", "46"),
    ("0", "38"),
    ("1", "47"),
    ("1", "2"),
    ("1", "38"),
    ("2", "46"),
    ("2", "42"),
    ("2", "14"),
    ("3", "49"),
    ("3", "7"),
    ("4", "45"),
    ("4", "38"),
    ("4", "17"),
    ("4", "24"),
    ("5", "34"),
    ("5", "13"),
    ("5", "6"),
    ("5", "35"),
    ("5", "18"),
    ("5", "48"),
    ("6", "47"),
    ("6", "24"),
    ("6", "14"),
    ("6", "40"),
    ("7", "40"),
    ("7", "24"),
    ("7", "46"),
    ("7", "21"),
    ("8", "14"),
    ("8", "15"),
    ("9", "13"),
    ("9", "16"),
    ("9", "40"),
    ("9", "12"),
    ("10", "48"),
    ("10", "23"),
    ("10", "40"),
    ("10", "15"),
    ("10", "43"),
    ("10", "29"),
    ("10", "34"),
    ("11", "32"),
    ("12", "35"),
    ("12", "36"),
    ("13", "42"),
    ("13", "36"),
    ("13", "41"),
    ("14", "32"),
    ("14", "26"),
    ("14", "49"),
    ("14", "43"),
    ("15", "17"),
    ("16", "38"),
    ("16", "34"),
    ("16", "35"),
    ("16", "46"),
    ("17", "21"),
    ("17", "24"),
    ("17", "44"),
    ("17", "40"),
    ("17", "48"),
    ("18", "40"),
    ("18", "27"),
    ("19", "40"),
    ("20", "49"),
    ("20", "25"),
    ("20", "45"),
    ("20", "31"),
    ("21", "48"),
    ("22", "23"),
    ("23", "39"),
    ("23", "25"),
    ("23", "48"),
    ("24", "29"),
    ("24", "38"),
    ("25", "31"),
    ("27", "37"),
    ("27", "44"),
    ("27", "38"),
    ("28", "37"),
    ("29", "34"),
    ("29", "40"),
    ("29", "41"),
    ("29", "33"),
    ("31", "32"),
    ("32", "48"),
    ("32", "38"),
    ("33", "49"),
    ("34", "44"),
    ("34", "46"),
    ("34", "43"),
    ("35", "44"),
    ("35", "47"),
    ("37", "47"),
    ("41", "45"),
    ("41", "43"),
    ("41", "49"),
    ("43", "47"),
];
const GNM_50_100_42_EXPECTED: &[(&str, &str)] = &[
    ("0", "17"),
    ("1", "38"),
    ("10", "43"),
    ("11", "32"),
    ("12", "35"),
    ("13", "42"),
    ("14", "8"),
    ("16", "9"),
    ("18", "27"),
    ("2", "46"),
    ("20", "25"),
    ("21", "48"),
    ("22", "23"),
    ("24", "29"),
    ("28", "37"),
    ("3", "49"),
    ("34", "5"),
    ("4", "45"),
    ("40", "7"),
    ("47", "6"),
];

#[test]
fn gnm_50_100_seed42() {
    let input = edges_to_str(GNM_50_100_42_EDGES);
    let got = run(&input);
    assert_eq!(
        got,
        expected(GNM_50_100_42_EXPECTED),
        "gnm(50,100,seed=42): matching mismatch vs networkx 3.6.1"
    );
}
