#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools as _;

#[argio::argio]
fn main(n: usize, k: i64, a: [i64; n]) {
    let v = a
        .into_iter()
        .filter(|ai| ai % k == 0)
        .map(|ai| ai / k)
        .collect_vec();
    for vi in v {
        print!("{} ", vi);
    }
    println!();
}
