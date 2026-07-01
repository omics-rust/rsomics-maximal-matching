//! Benchmark: compute-only (graph already loaded) on a gnm(5000, 15000)
//! graph, measuring `Graph::maximal_matching` against a Python-NetworkX
//! baseline of ~2.98ms per call on the same hardware.
//!
//! Run: cargo bench --bench matching

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rsomics_maximal_matching::Graph;

fn build_graph(n: usize, m: usize, seed: u64) -> Graph {
    // Deterministic gnm-style graph using a simple LCG. Node insertion
    // order matches first-appearance in the edge stream (same as
    // networkx gnm_random_graph → add_edges_from).
    use std::collections::HashSet;
    let mut g = Graph::new();

    // Generate m distinct edges deterministically
    let mut rng = seed;
    let lcg_next = |s: &mut u64| -> u64 {
        *s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        *s
    };

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut count = 0;
    while count < m {
        let u = (lcg_next(&mut rng) as usize) % n;
        let v = (lcg_next(&mut rng) as usize) % n;
        if u == v {
            continue;
        }
        let key = if u < v { (u, v) } else { (v, u) };
        if seen.insert(key) {
            g.add_edge(&u.to_string(), &v.to_string());
            count += 1;
        }
    }
    g
}

fn bench_matching(c: &mut Criterion) {
    // Build the graph once, then only time the matching computation.
    let g = build_graph(5000, 15000, 2024);

    let mut group = c.benchmark_group("maximal_matching");
    group.bench_function(BenchmarkId::new("gnm", "5k_nodes_15k_edges"), |b| {
        b.iter(|| {
            let m = g.maximal_matching();
            criterion::black_box(m)
        });
    });
    group.finish();
}

criterion_group!(benches, bench_matching);
criterion_main!(benches);
