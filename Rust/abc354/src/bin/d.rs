#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;
use rand::seq::SliceRandom;

#[argio::argio]
fn main() {
    let mut rng = rand::thread_rng();
    let n = 6;
    let mut a_list = (1..=n).map(|i| i * 10).collect_vec();
    a_list.shuffle(&mut rng);
    let mut c_list = (1..=n).map(|i| i * 100).collect_vec();
    c_list.shuffle(&mut rng);

    println!("{}", n);
    for (a, c) in a_list.iter().zip(c_list.iter()) {
        println!("{} {}", a, c);
    }

    println!("----------");

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
