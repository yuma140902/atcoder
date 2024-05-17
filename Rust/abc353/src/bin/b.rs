#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, k: i64, a: [i64; n]) -> i64 {
    let mut count = 0;
    let mut empty = k;
    let mut i = 0;

    while i < a.len() {
        if a[i] > empty {
            count += 1;
            empty = k;
        } else {
            empty -= a[i];
            i += 1;
        }
    }
    count += 1;
    count
}
