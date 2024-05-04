#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(a: [i64; 9], b: [i64; 8]) -> i64 {
    let a_sum = a.iter().sum::<i64>();
    let b_sum = b.iter().sum::<i64>();
    a_sum - b_sum + 1
}
