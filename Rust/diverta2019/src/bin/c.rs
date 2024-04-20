#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(n: usize, s: [Chars; n]) -> i64 {
    let mut ab = 0;
    let mut a = 0;
    let mut b = 0;
    let mut count = 0;

    for si in s {
        for i in 0..(si.len() - 1) {
            if si[i] == 'A' && si[i + 1] == 'B' {
                count += 1;
            }
        }

        if si[0] == 'B' && si[si.len() - 1] == 'A' {
            ab += 1;
        } else if si[0] == 'B' {
            b += 1;
        } else if si[si.len() - 1] == 'A' {
            a += 1;
        }
    }

    if ab == 0 {
        count + a.min(b)
    } else if a == 0 && b == 0 {
        count + ab - 1
    } else {
        count + ab + a.min(b)
    }
}
