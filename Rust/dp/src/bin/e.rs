#[allow(unused_imports)]
use std::collections::*;

const V_MAX: usize = 1001;

#[argio::argio]
fn main(n: usize, w: i64, wv: [(i64, i64); n]) -> i64 {
    // dp[i][v] : i-1番目までの品物から価値がvとなるように選んだときの重さの総和の最小値
    let mut dp = vec![vec![1000_i64; V_MAX]; n + 1];

    /*for v in 0..V_MAX {
        dp[0][v] = 0;
    }*/

    for i in 0..n {
        eprintln!("品物{}を追加するかどうか考えます。", i + 1);
        for vi in 0..V_MAX {
            if wv[i].1 <= vi as i64 {
                eprintln!("追加することができます");
                eprint!("追加するとdp[{}][{}]は{}から", i + 1, vi, dp[i + 1][vi]);
                dp[i + 1][vi] =
                    dp[i + 1][vi].min((dp[i][vi - wv[i].1 as usize]).saturating_add(wv[i].0));
                eprintln!("{}に変わります。", dp[i + 1][vi]);
            } else {
                eprintln!("追加することはできません。");
            }
            eprint!(
                "追加しない場合、dp[{}][{}]は{}から",
                i + 1,
                vi,
                dp[i + 1][vi]
            );
            dp[i + 1][vi] = dp[i + 1][vi].min(dp[i][vi]);
            eprintln!("{}に変わります。", dp[i + 1][vi]);
        }
    }

    let mut ans = i64::MIN;
    for v in 0..V_MAX {
        if dp[n][v] <= w {
            eprintln!(
                "ansを{}に更新します。なぜならdp[n][v] = {} <= wだからです。",
                v, dp[n][v]
            );
            ans = v as i64;
        }
    }

    ans
}
