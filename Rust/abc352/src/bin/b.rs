#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(s: Chars, t: Chars) {
    let mut j = 0;
    for i in 0..s.len() {
        while j < t.len() && s[i] != t[j] {
            j += 1;
        }
        print!("{} ", j + 1);
        j += 1;
    }
    println!();
}
