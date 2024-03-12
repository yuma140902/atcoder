#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(c: String) {
    let count: i32 = c.parse().unwrap();
    for _ in 0..count {
        print!("{}", c);
    }
    println!();
}
