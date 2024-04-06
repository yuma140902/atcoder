#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(s: Chars) -> i64 {
    let mut set = HashSet::new();
    for width in 0..=s.len() {
        for i in 0..s.len() - width {
            let sub = &s[i..=(i + width)];
            set.insert(sub);
        }
    }
    set.len() as i64
}
