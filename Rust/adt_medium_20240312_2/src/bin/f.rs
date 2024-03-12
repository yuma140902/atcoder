#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, m: usize, a: [i64; n]) -> i64 {
    let mut prefix_sum = vec![0; n + 1];
    prefix_sum[0] = 0;
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + a[i];
    }

    let mut left = 0;
    let mut sum: i64 = a[0..m]
        .iter()
        .enumerate()
        .map(|(i, &x)| x * (i + 1) as i64)
        .sum();

    let mut ans = sum;
    loop {
        left += 1;
        if left + m - 1 >= n {
            break;
        }

        sum += a[left + m - 1] * (m) as i64;
        sum -= prefix_sum[left - 1 + m] - prefix_sum[left - 1];

        ans = ans.max(sum);
    }

    ans
}
