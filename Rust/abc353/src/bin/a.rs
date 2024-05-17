#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, h: [i64; n]) -> i64 {
    for i in 1..h.len() {
        if h[i] > h[0] {
            return i as i64 + 1;
        }
    }
    return -1;
}
