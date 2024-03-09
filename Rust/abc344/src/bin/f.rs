#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Entry {
    dist: i64,
    money: i64,
    node: (usize, usize),
}

#[argio::argio]
fn main(n: usize, p: [[i64; n]; n], r: [[i64; n - 1]; n], d: [[i64; n]; n - 1]) {
    let mut heap = BinaryHeap::new();
    heap.push(Entry {
        dist: 0,
        money: 0,
        node: (0, 0),
    });

    let mut minimum = vec![vec![std::i64::MAX; n]; n];

    while let Some(entry) = heap.pop() {
        let i = entry.node.0;
        let j = entry.node.1;

        if i >= n || j >= n {
            continue;
        }

        minimum[i][j] = minimum[i][j].min(entry.dist);

        if j + 1 < n && entry.money >= r[i][j] {
            let next = (i, j + 1);
            if minimum[i][j + 1] > entry.dist + r[i][j] {
                heap.push(Entry {
                    dist: entry.dist + 1,
                    money: entry.money - r[i][j],
                    node: next,
                });
            }
        }

        if i + 1 < n && entry.money >= d[i][j] {
            let next = (i + 1, j);
            if minimum[i + 1][j] > entry.dist + d[i][j] {
                heap.push(Entry {
                    dist: entry.dist + 1,
                    money: entry.money - d[i][j],
                    node: next,
                });
            }
        }

        if j + 1 < n && entry.money < r[i][j] || i + 1 < n && entry.money < d[i][j] {
            heap.push(Entry {
                dist: entry.dist + 1,
                money: entry.money + p[i][j],
                node: entry.node,
            });
        }
    }

    let cost = minimum[n - 1][n - 1];
    println!("{}", cost);

    //println!("{}", routes[&node_indices[n * n - 1]]);
}
