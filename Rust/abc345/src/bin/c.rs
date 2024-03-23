#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

fn n_choose_2(n: i64) -> i64 {
    n * (n - 1) / 2
}

#[argio::argio]
fn main(s: Chars) -> i64 {
    let mut prev = None;
    let mut t = Vec::with_capacity(s.len());
    for &c in &s {
        if Some(c) != prev {
            t.push(c);
            prev = Some(c);
        }
    }
    dbg!(&t);
    let n = t.len() as i64;

    let mut hist = HashMap::new();
    for &c in &t {
        *hist.entry(c).or_insert(0) += 1;
    }

    if n == 1 {
        return 1;
    }

    let mut ans = n_choose_2(n);
    for &v in hist.values() {
        ans -= n_choose_2(v);
    }
    ans
}
