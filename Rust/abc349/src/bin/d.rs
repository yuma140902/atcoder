#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(l: u64, r: u64) {
    let v = solve(l, r);

    println!("{}", v.len());
    for (a, b) in v {
        println!("{} {}", a, b);
    }
}

fn find_i(l: u64, r: u64) -> i64 {
    let mut i = 60;
    loop {
        if l == 0 || l % (1u64 << i) == 0 {
            let j = l / (1u64 << i);
            if (1u64 << i) * (j + 1) <= r {
                return i;
            }
        }
        i -= 1;
    }
}

fn solve(mut l: u64, r: u64) -> Vec<(u64, u64)> {
    let mut v = vec![];

    while l < r {
        let i = find_i(l, r);
        let j = l / (1u64 << i);
        //eprintln!("{} = 2^{} * {}", l, i, j);
        let new_l = (1u64 << i) * (j + 1);
        v.push((l, new_l));
        l = new_l;
    }

    v
}
