#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: u64, mut m: u64, a: [u64; n]) -> u64 {
    let mut a = a.into_iter().collect::<BinaryHeap<_>>();

    while m > 0 {
        let max = a.pop().unwrap();
        a.push(max / 2);
        m -= 1;
    }

    a.iter().sum()
}
