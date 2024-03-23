#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, k: i64, a: [i64; n]) -> i64 {
    let mut set = HashSet::new();

    let mut sum = 0;
    for ai in a {
        if ai <= k && set.insert(ai) {
            sum += ai;
        }
    }

    k * (k + 1) / 2 - sum
}
