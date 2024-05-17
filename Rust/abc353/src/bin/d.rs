#[allow(unused_imports)]
use std::collections::*;

use ac_library::ModInt998244353 as mint;
use itertools::Itertools;

fn num_digits(n: i64) -> u32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

#[argio::argio]
fn main(n: usize, a: [i64; n]) -> i64 {
    let ad = a
        .into_iter()
        .map(|ai| (ai, 10_i64.pow(num_digits(ai))))
        .collect_vec();

    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + ad[i].0;
    }

    let mut sum = mint::raw(0);
    for i in 1..n {
        sum += mint::new(prefix[i]) * mint::new(ad[i].1) + mint::new(ad[i].0) * i as i64;
    }
    sum.val() as i64
}
