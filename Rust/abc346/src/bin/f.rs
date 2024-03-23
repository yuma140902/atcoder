#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;
use proconio::marker::Chars;

fn g(t: &[char], k: i64) -> String {
    t.iter()
        .flat_map(|&ti| std::iter::repeat(ti).take(k as usize))
        .collect()
}

fn f(s: &[char], n: i64) -> String {
    s.iter()
        .cycle()
        .take(s.len() * n as usize)
        .copied()
        .collect()
}

#[argio::argio]
fn main(n: i64, s: Chars, t: Chars) -> i64 {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < t.len() {
        while j < s.len() {
            if s[j] == t[i] {
                k += 1;
                if k == n {
                    i += 1;
                    j = 0;
                }
            }
            j += 1;
        }
        i += 1;
    }

    k
}
