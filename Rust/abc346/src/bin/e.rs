#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;

#[argio::argio]
fn main(h: usize, w: usize, m: usize, ops: [(i64, usize, i64); m]) {
    let mut colored_rows = HashSet::new();
    let mut colored_columns = HashSet::new();
    let mut counters: HashMap<i64, i64> = HashMap::new();
    for (t, a, x) in ops.into_iter().rev() {
        if t == 1 {
            if colored_rows.insert(a - 1) {
                let inc = w as i64 - colored_columns.len() as i64;
                counters.entry(x).and_modify(|e| *e += inc).or_insert(inc);
            }
        } else {
            if colored_columns.insert(a - 1) {
                let inc = h as i64 - colored_rows.len() as i64;
                counters.entry(x).and_modify(|e| *e += inc).or_insert(inc);
            }
        }
    }

    let colored_items = counters.values().sum::<i64>();
    let zeros = ((h * w) as i64 - colored_items).max(0);
    counters
        .entry(0)
        .and_modify(|e| *e += zeros)
        .or_insert(zeros);

    let counters = counters
        .into_iter()
        .filter(|(_, v)| *v > 0)
        .sorted_by_key(|(k, _)| *k)
        .collect::<Vec<_>>();
    println!("{}", counters.len());
    for (k, v) in counters {
        println!("{} {}", k, v);
    }
}
