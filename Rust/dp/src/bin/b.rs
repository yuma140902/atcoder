#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, k: usize, h: [i64; n]) -> i64 {
    let mut dp = vec![i64::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        for j in 1..=k {
            if i >= j {
                dp[i] = dp[i].min(dp[i - j] + (h[i - j] - h[i]).abs());
            }
        }
    }
    dp[n - 1]
}
