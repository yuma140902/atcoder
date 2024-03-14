#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, h: [i64; n]) -> i64 {
    let mut dp = vec![i64::MAX; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();

    for i in 2..n {
        dp[i] = (dp[i - 1] + (h[i] - h[i - 1]).abs()).min(dp[i - 2] + (h[i] - h[i - 2]).abs());
    }

    dp[n - 1]
}
