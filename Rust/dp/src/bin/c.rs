#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, abc: [[i64; 3]; n]) -> i64 {
    let mut dp = vec![[i64::MIN; 3]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    dp[0][2] = 0;

    for i in 1..(n + 1) {
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    dp[i][k] = dp[i][k].max(dp[i - 1][j] + abc[i - 1][k]);
                }
            }
        }
    }

    *dp[n].iter().max().unwrap()
}
