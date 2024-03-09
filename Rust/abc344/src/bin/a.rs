#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools as _;

#[argio::argio]
fn main(s: String) -> String {
    let ss = s.split("|").collect_vec();
    format!("{}{}", ss[0], ss[2])
}
