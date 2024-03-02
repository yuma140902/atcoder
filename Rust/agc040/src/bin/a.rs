use itertools::Itertools;

#[argio::argio]
fn main(s: String) -> i64 {
    let s = s.chars().collect_vec();
    let mut lt = vec![-1; s.len() + 1];
    let mut gt = vec![-1; s.len() + 1];

    lt[0] = 0;
    for i in 0..s.len() {
        if s[i] == '<' {
            lt[i + 1] = lt[i] + 1;
        } else {
            lt[i + 1] = 0;
        }
    }

    *gt.last_mut().unwrap() = 0;
    for i in (0..s.len()).rev() {
        if s[i] == '>' {
            gt[i] = gt[i + 1] + 1;
        } else {
            gt[i] = 0;
        }
    }

    lt.iter().zip(gt.iter()).map(|(l, g)| l.max(g)).sum()
}
