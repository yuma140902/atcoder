#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(n: usize, d: usize, s: [Chars; n]) -> i32 {
    let mut schedule = vec![true; d];

    for i in 0..d {
        for j in 0..n {
            if s[j][i] == 'x' {
                schedule[i] = false;
            }
        }
    }

    let mut prefix_sum = vec![0; d + 1];

    prefix_sum[0] = 0;
    for i in 1..(d + 1) {
        if !schedule[i - 1] {
            prefix_sum[i] = 0;
        } else {
            prefix_sum[i] = prefix_sum[i - 1] + schedule[i - 1] as i32;
        }
    }

    prefix_sum.into_iter().max().unwrap()
}
