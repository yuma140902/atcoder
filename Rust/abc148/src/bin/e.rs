#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(mut n: u64) -> u64 {
    if n % 2 == 1 {
        return 0;
    }

    let n = n / 2;
    let mut div = 5;
    let mut ret = 0;
    loop {
        ret += n / div;
        div *= 5;
        if div > n {
            break;
        }
    }

    ret
}
