#[allow(unused_imports)]
use std::collections::*;

use amplify::confinement::Collection;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Card {
    strength: i64,
    cost: i64,
}

#[derive(Debug, Clone)]
struct Entry {
    card: Card,
    alive: bool,
}

#[argio::argio]
fn main(n: usize, ac: [(i64, i64); n]) {
    //let mut cards = Vec::with_capacity(n);
    let mut a_list = Vec::with_capacity(n);
    let mut c_list = Vec::with_capacity(n);
    for (i, (a, c)) in ac.iter().enumerate() {
        a_list.push(*a);
        c_list.push(*c);
    }

    let mut removed = vec![false; n];
    for x in 0..n {
        for y in 0..n {
            if a_list[x] > a_list[y] && c_list[x] < c_list[y] {
                removed[y] = true;
            }
        }
    }

    println!("{}", removed.iter().filter(|&&x| !x).count());
    println!(
        "{}",
        removed
            .iter()
            .enumerate()
            .filter(|(_, &x)| !x)
            .map(|(i, _)| format!("{}", i + 1))
            .join(" ")
    );
}
