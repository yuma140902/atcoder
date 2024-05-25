#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(mut h: i64) -> i64 {
    let mut d = 1;
    let mut count = 0;
    let mut k = 0;
    while k <= h {
        k += d;
        d *= 2;
        count += 1;
    }
    count
}
