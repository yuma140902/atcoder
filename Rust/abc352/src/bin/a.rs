#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: i64, x: i64, y: i64, z: i64) -> String {
    let x_ = x.min(y);
    let y_ = x.max(y);
    if x_ <= z && z <= y_ {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
