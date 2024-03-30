#[allow(unused_imports)]
use std::collections::*;

use num::Integer;

fn factorization(mut n: i64) -> HashMap<i64, u64> {
    let mut i = 2;
    let mut map = HashMap::new();
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            map.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
        i += 1;
    }

    if n != 0 && n != 1 {
        map.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }

    map
}

#[argio::argio]
fn main(a: i64, b: i64) -> i64 {
    let gcd = a.gcd(&b);
    let factors = factorization(gcd);

    factors.keys().count() as i64 + 1
}
