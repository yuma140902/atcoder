#[allow(unused_imports)]
use std::collections::*;

use proconio::input;

fn step(t: &Vec<char>, bags: &Vec<Vec<Vec<char>>>, i: usize, cost: u64, s: Vec<char>) -> u64 {
    let mut costs = vec![];

    if i == bags.len() {
        if t == &s {
            return cost;
        }
        return std::u64::MAX;
    }

    if t.len() < s.len() {
        return std::u64::MAX;
    }

    for str_in_bag in &bags[i] {
        if t[s.len()..].starts_with(&str_in_bag) {
            let mut s = s.clone();
            s.extend(str_in_bag);
            costs.push(step(t, bags, i + 1, cost + 1, s));
        }
    }
    costs.push(step(t, bags, i + 1, cost, s));

    costs.into_iter().min().unwrap()
}

fn main() {
    input! {
        t: String,
        n: usize
    };

    let t = t.chars().collect::<Vec<_>>();
    let mut bags = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            a: usize,
            ss: [String; a]
        };

        let mut ss = ss
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        ss.sort_by(|a, b| b.len().cmp(&a.len()));
        bags.push(ss);
    }

    let cost = step(&t, &bags, 0, 0, vec![]);
    if cost == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", cost);
    }
}

