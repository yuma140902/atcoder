#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, q: usize, x: [usize; q]) {
    let mut a = vec![0; n];
    let mut s = vec![false; n];
    let mut s_len = 0;

    for xi in x {
        if s[xi] {
            s[xi] = false;
            s_len -= 1;
        } else {
            s[xi] = true;
            s_len += 1;
        }
    }
    todo!();
}
