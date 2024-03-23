#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(x: i128) -> i128 {
    if x >= 0 {
        (x + 9) / 10
    } else {
        x / 10
    }
}
