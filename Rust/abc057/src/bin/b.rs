#[allow(unused_imports)]
use std::collections::*;

fn distance((px, py): (i64, i64), (qx, qy): (i64, i64)) -> i64 {
    (px - qx).abs() + (py - qy).abs()
}

#[argio::argio]
fn main(n: usize, m: usize, ab: [(i64, i64); n], cd: [(i64, i64); m]) {
    for i in 0..n {
        let minimum = cd
            .iter()
            .enumerate()
            .map(|(j, p)| (j, distance(ab[i], *p)))
            .min_by_key(|(_, d)| *d)
            .unwrap();
        println!("{}", minimum.0 + 1);
    }
}
