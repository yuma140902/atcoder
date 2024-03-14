#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, w: usize, wv: [(i64, i64); n]) -> i64 {
    // dp[i][w] : i-1番目までの品物から重さがちょうどwになるように選んだときの価値の総和の最大値
    let mut dp = vec![vec![i64::MIN; w + 1]; n + 1];

    for wi in 0..(w + 1) {
        dp[0][wi] = 0;
    }
    for i in 0..n {
        for wi in 0..(w + 1) {
            if wi as i64 >= wv[i].0 {
                dp[i + 1][wi] = dp[i + 1][wi].max(dp[i][wi - wv[i].0 as usize] + wv[i].1);
            }
            dp[i + 1][wi] = dp[i + 1][wi].max(dp[i][wi]);
        }
    }

    dp[n][w]
}
