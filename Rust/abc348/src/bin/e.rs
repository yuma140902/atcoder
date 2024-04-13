use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

use amplify::confinement::Collection;
use petgraph::graph::UnGraph;

#[argio::argio]
fn main(n: usize, ab: [(i64, i64); n], c: [i64; n]) {
    let mut c_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut distance_heap: BinaryHeap<i64> = BinaryHeap::new();

    let mut tree = UnGraph::<(), i64>::new_undirected();
    let mut nodes = Vec::with_capacity(n);
    for _i in 0..n {
        nodes.push(tree.add_node(()));
    }
    for (a, b) in ab {
        tree.add_edge(nodes[a as usize], nodes[b as usize], 1);
    }
}
