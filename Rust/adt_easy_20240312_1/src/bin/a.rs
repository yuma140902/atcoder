#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(s: String) -> i32 {
    let kinds = s.chars().collect::<HashSet<_>>().len() as i32;

    if kinds == 3 {
        3 * 2
    } else if kinds == 2 {
        3 * 2 / 2
    } else {
        1
    }
}
