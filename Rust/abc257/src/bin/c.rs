use std::process::exit;

use itertools::{Itertools, MinMaxResult};
#[allow(unused_imports)]
use proconio::input;

#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! debug (($arg:expr) => (dbg!($arg)));

#[allow(unused_macros)]
#[cfg(not(debug_assertions))]
macro_rules! debug (($arg:expr) => ($arg));

fn main() {
    input! {
        n: usize,
        s: String,
        w: [i32; n]
    }

    //    let (min_w, max_w) = match w.iter().minmax() {
    //        MinMaxResult::NoElements => panic!(),
    //        MinMaxResult::OneElement(mm) => (*mm, *mm),
    //        MinMaxResult::MinMax(min, max) => (*min, *max),
    //    };

    let is_adult = s.chars().map(|c| c == '1').collect_vec();
    let w = w
        .into_iter()
        .enumerate()
        .map(|(i, w)| (w, is_adult[i]))
        .collect_vec();

    debug!(&w);

    let mut ans = 0;

    let min_adult = w
        .iter()
        .filter(|(_, is_adult)| *is_adult)
        .map(|(w, _)| *w)
        .min()
        .unwrap_or_else(|| {
            println!("{}", n);
            exit(0)
        });

    let max_child = w
        .iter()
        .filter(|(_, is_adult)| !*is_adult)
        .map(|(w, _)| *w)
        .max()
        .unwrap_or_else(|| {
            println!("{}", n);
            exit(0)
        });

    for th in min_adult..=(max_child + 1) {
        ans = takahashi(&w, th).max(ans);
    }

    println!("{}", ans);
}

fn takahashi(w: &Vec<(i32, bool)>, th: i32) -> usize {
    w.iter()
        .filter(|(w, is_adult)| (th <= *w) == *is_adult)
        .count()
}
