#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(
    n: usize,
    mut a: [u64; n],
    m: usize,
    mut b: [u64; m],
    l: usize,
    mut c: [u64; l],
    q: usize,
    x: [u64; q],
) {
    let mut map = HashSet::new();

    for ai in &a {
        for bi in &b {
            for ci in &c {
                map.insert(ai + bi + ci);
            }
        }
    }

    for xi in &x {
        if map.contains(&xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
