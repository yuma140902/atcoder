#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(s: String) {
    let mut b0 = None;
    let mut b1 = None;
    let mut k = None;
    let mut r0 = None;
    let mut r1 = None;

    for (i, c) in s.chars().enumerate() {
        if c == 'B' && b0.is_none() {
            b0 = Some(i);
        } else if c == 'B' {
            b1 = Some(i);
        } else if c == 'K' {
            k = Some(i);
        } else if c == 'R' && r0.is_none() {
            r0 = Some(i);
        } else if c == 'R' {
            r1 = Some(i);
        }
    }

    if b0.unwrap() % 2 != b1.unwrap() % 2 && (r0.unwrap() < k.unwrap() && k.unwrap() < r1.unwrap())
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
