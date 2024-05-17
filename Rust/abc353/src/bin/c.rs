#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, mut a: [i64; n]) -> i64 {
    a.sort();
    let sum = a.iter().sum::<i64>();
    let mut result = sum * (n as i64 - 1);

    let mut left = 0;
    let mut right = n - 1;
    let mut overflow_count = 0;
    while left < n && a[left] + a[right] < 1_0000_0000 {
        left += 1;
    }
    while left < right && right < n {
        if a[left] + a[right] >= 1_0000_0000 {
            overflow_count += right - left;
            right -= 1;
        } else {
            left += 1;
        }
    }

    let mut count = 0;
    while result > 1_0000_0000 && count < overflow_count {
        count += 1;
        result -= 1_0000_0000;
    }
    result
}
