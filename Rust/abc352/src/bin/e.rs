use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut queries = BinaryHeap::new();

    for _ in 0..m {
        input! {
            k: usize,
            c: i64,
            a: [usize; k]
        }
        queries.push((Reverse(c), a));
    }

    let mut dsu = Dsu::new(n);
    let mut total_cost = 0;

    while let Some((Reverse(cost), nodes)) = queries.pop() {
        let current_root = nodes[0];

        for node in &nodes[1..] {
            if !dsu.same(current_root - 1, *node - 1) {
                dsu.merge(current_root - 1, *node - 1);
                total_cost += cost;
            }
        }
    }

    if dsu.groups().len() == 1 {
        println!("{}", total_cost);
    } else {
        println!("-1");
    }
}
