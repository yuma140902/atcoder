use itertools::Itertools;

fn is_even_str(s: &[char]) -> bool {
    let s_len = s.len();
    s.iter()
        .take(s_len / 2)
        .zip(s.iter().skip(s_len / 2))
        .all(|(a, b)| a == b)
}

#[argio::argio]
fn main(s: String) -> usize {
    let s = s.chars().collect_vec();
    let mut len = s.len() - if s.len() % 2 == 0 { 2 } else { 1 };

    loop {
        if is_even_str(&s[0..len]) {
            return len;
        }
        len -= 2;
        if len <= 0 {
            break;
        }
    }

    0
}
