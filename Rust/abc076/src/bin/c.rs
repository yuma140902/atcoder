use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;

const UNRESTORABLE: &'static str = "UNRESTORABLE";

fn matches(s: &[char], t: &[char]) -> bool {
    s.iter().zip(t.iter()).all(|(c, d)| *c == '?' || *c == *d)
}

fn restore(mut s_prime: Vec<char>, t: &[char], t_start_index: usize) -> String {
    for i in 0..s_prime.len() {
        if i < t_start_index {
            if s_prime[i] == '?' {
                s_prime[i] = 'a';
            }
        } else if i - t_start_index < t.len() {
            s_prime[i] = t[i - t_start_index];
        } else {
            if s_prime[i] == '?' {
                s_prime[i] = 'a';
            }
        }
    }
    s_prime.into_iter().collect::<String>()
}

#[argio::argio]
fn main(s_prime: String, t: String) -> String {
    let s_prime = s_prime.chars().collect_vec();
    let t = t.chars().collect_vec();

    let mut answers = BinaryHeap::new();

    let mut index = s_prime.len() as i64 - t.len() as i64;
    while index >= 0 {
        if matches(&s_prime[(index as usize)..], &t) {
            answers.push(Reverse(restore(s_prime.clone(), &t, index as usize)));
        }
        index -= 1;
    }

    if let Some(answer) = answers.pop() {
        answer.0
    } else {
        UNRESTORABLE.to_string()
    }
}
