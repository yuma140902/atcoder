#[allow(unused_imports)]
use std::collections::*;

use petgraph::{
    algo::TarjanScc,
    graph::{Graph, NodeIndex},
    Undirected,
};

#[argio::argio]
fn main(n: usize, m: usize, ab: [(usize, usize); m]) -> i64 {
    let mut edges = Vec::with_capacity(m * 2);
    for &(a, b) in &ab {
        edges.push((a, b));
        edges.push((b, a));
    }

    let graph: Graph<(), (), Undirected, _> = Graph::from_edges(edges.into_iter());

    let mut tarjan_scc = TarjanScc::new();
    let mut result: Vec<HashSet<_>> = Vec::new();
    tarjan_scc.run(&graph, |scc| {
        result.push(scc.iter().map(|node_idx| node_idx.index()).collect())
    });

    let mut count = 0;

    for node_set in &result {
        let n = node_set.len() as i64;
        count += (n - 1) * n / 2;
    }

    count -= m as i64;

    count
}

