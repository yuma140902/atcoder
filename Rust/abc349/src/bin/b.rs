#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(s: Chars) -> String {
    let mut map = HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut chars = HashMap::new();
    for v in map.values() {
        *chars.entry(v).or_insert(0) += 1;
    }

    for (k, v) in chars {
        if v != 0 && v != 2 {
            return "No".to_string();
        }
    }

    "Yes".to_string()
}
