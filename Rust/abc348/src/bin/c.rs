#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, ac: [(i64, i64); n]) {
    let mut mins: HashMap<i64, i64> = HashMap::new();

    for (a, c) in ac {
        mins.entry(c).and_modify(|x| *x = (*x).min(a)).or_insert(a);
    }

    let ans = mins.values().max().unwrap();
    println!("{}", ans);
}
