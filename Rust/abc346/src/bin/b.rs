#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;

fn check(s: &[char], w: i64, b: i64) -> bool {
    let count_w = s.iter().filter(|&&c| c == 'w').count() as i64;
    let count_b = s.iter().filter(|&&c| c == 'b').count() as i64;
    count_w == w && count_b == b
}

#[argio::argio]
fn main(w: i64, b: i64) -> &'static str {
    let v = "wbwbwwbwbwbw".chars().cycle().take(200).collect_vec();
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            if check(&v[i..j], w, b) {
                return "Yes";
            }
        }
    }
    "No"
}
