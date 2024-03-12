#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(s: String) {
    let s_len = s.len();
    let s = s.chars().collect::<HashSet<_>>();
    let has_lower = s.iter().any(|c| c.is_lowercase());
    let has_upper = s.iter().any(|c| c.is_uppercase());

    if s_len == s.iter().count() && has_lower && has_upper {
        println!("Yes");
    } else {
        println!("No");
    }
}
