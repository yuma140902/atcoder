#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, a: i64, b: i64, d: [i64; n]) -> &'static str {
    let mut d = d
        .into_iter()
        .map(|di| di % (a + b))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    d.sort();

    let mut diff_max = i64::MIN;

    if d.len() == 1 {
        return "Yes";
    }

    for i in 1..d.len() {
        let diff = (d[i] - d[i - 1]).abs() + 1;
        diff_max = diff_max.max(diff);
        dbg!(diff_max);
    }
    let diff = a + b - ((d[0] - d[d.len() - 1]).abs()) + 1;
    diff_max = diff_max.max(diff);
    dbg!(diff_max);

    if diff_max >= b + 2 {
        "Yes"
    } else {
        "No"
    }
}
