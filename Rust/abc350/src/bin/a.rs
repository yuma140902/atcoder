#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(s: String) -> String {
    let n = s[3..].parse::<i32>().unwrap();
    if 1 <= n && n <= 315 || 317 <= n && n <= 349 {
        return "Yes".to_string();
    }
    "No".to_string()
}
