#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, q: usize, t: [usize; q]) -> i64 {
    let mut a = vec![true; n];

    for ti in t {
        a[ti - 1] = !a[ti - 1];
    }

    a.iter().filter(|&&x| x).count() as i64
}
