#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: i64) {
    for i in 0..n {
        if i % 3 == 2 {
            print!("x");
        } else {
            print!("o");
        }
    }
    println!();
}
