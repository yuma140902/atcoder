#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, mut sc: [(String, i64); n]) -> String {
    sc.sort_by_key(|(s, _)| s.clone());
    let t = sc.iter().map(|(_, c)| c).sum::<i64>();
    let no = t % n as i64;
    sc[no as usize].0.clone()
}
