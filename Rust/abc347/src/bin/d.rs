#[allow(unused_imports)]
use std::collections::*;

fn popcount(mut n: u64) -> i64 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count as i64
}

fn num_from_ones(n: i64) -> u64 {
    let mut result = 0;
    for i in 0..n {
        result <<= 1;
        result |= 1;
    }
    result
}

fn one_to_zero(bits: u64, n: i64) -> Option<u64> {
    if n < 0 || 60 < n || popcount(bits) < n {
        eprintln!(
            "bits = 0b{:060b} から{n}個の1を取り除くことはできない",
            bits
        );
        return None;
    }

    Some(bits >> n)
}

fn zero_to_one(bits: u64, n: i64) -> Option<u64> {
    if n < 0 || 60 < n || 60 - popcount(bits) < n {
        eprintln!(
            "bits = 0b{:060b} から{n}個の0を取り除くことはできない",
            bits
        );
        return None;
    }

    let mut ones = 0_u64;
    for i in 0..n {
        ones |= 1_u64 << 60;
        ones >>= 1;
    }
    Some(bits | ones)
}

#[argio::argio]
fn main(a: i64, b: i64, c: u64) {
    let mut x = num_from_ones(a);
    let mut y = 0;
    for i in 0..60 - a {
        let y_ = c ^ x;
        eprintln!("y_ = 0b{:060b}", y_);
        eprintln!("popcount(y_) = {}", popcount(y_));
        if popcount(y_) == b {
            y = y_;
            break;
        }
        x <<= 1;
    }
    eprintln!("c = 0b{:060b}", c);
    eprintln!("x = 0b{:060b}", x);
    eprintln!("y = 0b{:060b}", y);
    eprintln!("check1: 0<=x<2^60: {}", x < 1 << 60);
    eprintln!("check2: 0<=y<2^60: {}", y < 1 << 60);
    eprintln!("check3: x xor y == c: {}", x ^ y == c);
    eprintln!("check4: popcount(x) == a: {}", popcount(x) == a);
    eprintln!("check5: popcount(y) == b: {}", popcount(y) == b);
    todo!();
}
