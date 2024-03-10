#[allow(unused_imports)]
use std::collections::*;

const fn choose(n: i64, r: i64) -> i64 {
    if r < 0 || r > n {
        0
    } else {
        let r = if r < n - r { r } else { n - r };
        let mut ret = 1;
        let mut i = 0;
        loop {
            if i == r {
                break;
            }
            ret *= n - i;
            i += 1;
        }
        i = 0;
        loop {
            if i == r {
                break;
            }
            ret /= i + 1;
            i += 1;
        }
        ret
    }
}

const fn num_neqs(digits: u32) -> i64 {
    let mut ret = 10_i64.pow(digits);
    let digits = digits as i64;
    let mut sign = -1;
    let mut i = 1;
    loop {
        if digits - i - 1 < 0 {
            break;
        }
        ret += sign * choose(digits - 1, i) * 10_i64.pow((digits - i - 1) as u32);
        i += 1;
        sign *= -1;
    }
    ret
}

const NEQS: [i64; 12] = [
    num_neqs(1),
    num_neqs(2),
    num_neqs(3),
    num_neqs(4),
    num_neqs(5),
    num_neqs(6),
    num_neqs(7),
    num_neqs(8),
    num_neqs(9),
    num_neqs(10),
    num_neqs(11),
    num_neqs(12),
];

fn get_digits(nth: i64) -> (u32, i64) {
    let mut base = 0;
    for i in 0..12 {
        if nth <= base {
            return (i as u32 + 1, nth - base);
        }
        base += NEQS[i];
    }
    (13, nth - base)
}

#[argio::argio]
fn main(t: usize, cases: [i64; t]) {
    dbg!(&NEQS);
    for case in cases {
        let digits = get_digits(case);
        println!("{:?}", digits);
    }
}
