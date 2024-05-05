#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: i64, ab: [(i64, i64); n]) -> i64 {
    let max_face = ab.iter().map(|&(a, b)| b - a).max().unwrap();
    ab.iter().map(|&(a, _)| a).sum::<i64>() + max_face
}
