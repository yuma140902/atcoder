use std::collections::HashSet;

use itertools::Itertools;

#[argio::argio]
fn main(_n: usize, s: String) -> u32 {
    let s = s.chars().collect_vec();

    let mut is_first = vec![false; s.len()];
    let mut is_last = vec![false; s.len()];

    let mut chars = HashSet::new();
    for i in 0..s.len() {
        if chars.insert(s[i]) {
            is_first[i] = true;
        }
    }

    chars.clear();
    for i in (0..s.len()).rev() {
        if chars.insert(s[i]) {
            is_last[i] = true;
        }
    }

    let mut max = 0;
    let mut depth = 0;
    for i in 0..s.len() {
        if is_first[i] {
            depth += 1;
        }
        if is_last[i] {
            depth -= 1;
        }
        max = max.max(depth);
    }

    max
}
