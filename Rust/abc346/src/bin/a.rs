#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, a: [i64; n]) {
    for i in 0..(n - 1) {
        let b = a[i] * a[i + 1];
        print!("{} ", b);
    }
    println!();
}
