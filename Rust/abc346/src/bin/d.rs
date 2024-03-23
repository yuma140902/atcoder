#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[argio::argio]
fn main(n: usize, s: Chars, c: [i64; n]) -> i64 {
    if n == 2 {
        if s[0] != s[1] {
            return c[0].min(c[1]);
        } else {
            return 0;
        }
    }

    // dpf0[i+1] : 最初の文字を0として順方向にi文字まで置き換えたときのコストの総和
    let mut dpf0 = vec![0; n + 1];
    let mut dpf1 = vec![0; n + 1];
    let mut dpb0 = vec![0; n + 1];
    let mut dpb1 = vec![0; n + 1];

    let mut sign = 1;
    for i in 0..n {
        if sign > 0 && s[i] == '0' || sign < 0 && s[i] == '1' {
            dpf0[i + 1] = dpf0[i] + c[i];
        } else {
            dpf0[i + 1] = dpf0[i];
        }
        if sign > 0 && s[i] == '1' || sign < 0 && s[i] == '0' {
            dpf1[i + 1] = dpf1[i] + c[i];
        } else {
            dpf1[i + 1] = dpf1[i];
        }
        sign *= -1;
    }

    sign = 1;
    for i in 0..n {
        if sign > 0 && s[n - 1 - i] == '0' || sign < 0 && s[n - 1 - i] == '1' {
            dpb0[i + 1] = dpb0[i] + c[n - 1 - i];
        } else {
            dpb0[i + 1] = dpb0[i];
        }
        if sign > 0 && s[n - 1 - i] == '1' || sign < 0 && s[n - 1 - i] == '0' {
            dpb1[i + 1] = dpb1[i] + c[n - 1 - i];
        } else {
            dpb1[i + 1] = dpb1[i];
        }
        sign *= -1;
    }

    if n % 2 == 0 {
        ((0..(n - 1))
            .map(|i| dpf0[i + 1] + dpb0[n - i - 1])
            .min()
            .unwrap())
        .min(
            (0..(n - 1))
                .map(|i| dpf1[i + 1] + dpb1[n - i - 1])
                .min()
                .unwrap(),
        )
    } else {
        ((0..(n - 1))
            .map(|i| dpf0[i + 1] + dpb1[n - i - 1])
            .min()
            .unwrap())
        .min(
            (0..(n - 1))
                .map(|i| dpf1[i + 1] + dpb0[n - i - 1])
                .min()
                .unwrap(),
        )
    }
}
