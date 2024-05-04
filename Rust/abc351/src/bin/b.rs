#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(n: usize, a: [Chars; n], b: [Chars; n]) {
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
    unreachable!()
}
