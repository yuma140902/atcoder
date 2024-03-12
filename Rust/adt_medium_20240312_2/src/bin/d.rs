#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, a: [i32; 4 * n - 1]) -> i32 {
    let mut map = HashMap::new();

    for ai in a {
        map.entry(ai).and_modify(|e| *e += 1).or_insert(1);
    }

    for (k, v) in map {
        if v == 3 {
            return k;
        }
    }

    unreachable!();
}
