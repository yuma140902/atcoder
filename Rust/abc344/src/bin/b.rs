#[allow(unused_imports)]
use std::collections::*;

fn main() {
    let mut v = vec![];
    loop {
        proconio::input! {
            a: i64
        }
        v.push(a);
        if a == 0 {
            break;
        }
    }

    for a in v.into_iter().rev() {
        println!("{}", a);
    }
}
