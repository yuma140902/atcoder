#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, a: [i64; n - 1]) -> i64 {
    -a.into_iter().sum::<i64>()
}
