#[allow(unused_imports)]
use std::collections::*;

fn num_digits(mut n: i64) -> i64 {
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

#[argio::argio]
fn main(n: i64) -> i64 {
    let mut min_digits = std::i64::MAX;
    let mut a = 1;
    while a * a <= n {
        if n % a == 0 {
            let b = n / a;
            min_digits = min_digits.min(num_digits(a.max(b)));
        }
        a += 1;
    }
    min_digits
}
