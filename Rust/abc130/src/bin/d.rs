#[allow(unused_imports)]
use std::collections::*;

use superslice::Ext;

#[argio::argio]
fn main(n: usize, k: u64, a: [u64; n]) -> usize {
    let mut b = vec![0; n + 1];

    b[0] = 0;
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }

    dbg!(&b);
    let mut count = 0;
    for i in 0..b.len() {
        let c = b[i..].lower_bound(&(k + b[i]));
        if c < b.len() - i {
            count += b.len() - i - c;
        }
    }

    count
}
