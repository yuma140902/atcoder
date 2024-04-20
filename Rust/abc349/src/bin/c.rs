#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;
use proconio::marker::Chars;

fn is_type_a(s: &[char], t: &[char]) -> bool {
    let mut i = 0;
    loop {
        if i >= s.len() {
            return false;
        }
        if s[i] == t[0] {
            let mut j = i + 1;
            loop {
                if j >= s.len() {
                    return false;
                }
                if s[j] == t[1] {
                    let mut k = j + 1;
                    loop {
                        if k >= s.len() {
                            return false;
                        }
                        if s[k] == t[2] {
                            return true;
                        }
                        k += 1;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
}

fn is_type_b(s: &[char], t: &[char]) -> bool {
    if t[2] != 'X' {
        return false;
    }
    let mut i = 0;
    loop {
        if i >= s.len() {
            return false;
        }
        if s[i] == t[0] {
            let mut j = i + 1;
            loop {
                if j >= s.len() {
                    return false;
                }
                if s[j] == t[1] {
                    return true;
                }
                j += 1;
            }
        }
        i += 1;
    }
}

fn search(s: &[char], c: char) -> Option<usize> {
    for i in 0..s.len() {
        if s[i] == c {
            return Some(i);
        }
    }
    None
}

#[argio::argio]
fn main(s: Chars, t: Chars) -> String {
    let s = s.into_iter().map(|c| c.to_ascii_uppercase()).collect_vec();
    if is_type_b(&s, &t) || is_type_a(&s, &t) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
