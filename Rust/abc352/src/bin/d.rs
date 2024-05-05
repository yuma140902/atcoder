use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

use ac_library::{Max, Min, Segtree};
use itertools::Itertools;

#[argio::argio]
fn main(n: usize, k: usize, p: [i64; n]) -> i64 {
    let q = p
        .iter()
        .enumerate()
        .sorted_by_key(|&(_, x)| x)
        .map(|(i, _)| i)
        .collect_vec();

    let min_segtree = Segtree::<Min<usize>>::from(q.clone());
    let max_segtree = Segtree::<Max<usize>>::from(q);

    let mut min_ans = i64::MAX;

    for i in 0..n - k + 1 {
        let min = min_segtree.prod(i..i + k);
        let max = max_segtree.prod(i..i + k);
        min_ans = min_ans.min(max as i64 - min as i64);
    }

    min_ans
}
