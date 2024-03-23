#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(s: String) -> &'static str {
    let regex = regex::Regex::new("^<=+>$").unwrap();
    if regex.is_match(&s) {
        "Yes"
    } else {
        "No"
    }
}
