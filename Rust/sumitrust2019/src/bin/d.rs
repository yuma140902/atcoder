#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

fn digit1(n: i64) -> char {
    (n % 10 + '0' as i64) as u8 as char
}

fn digit2(n: i64) -> char {
    ((n / 10) % 10 + '0' as i64) as u8 as char
}

fn digit3(n: i64) -> char {
    ((n / 100) % 10 + '0' as i64) as u8 as char
}

#[argio::argio]
fn main(n: i64, s: Chars) -> i64 {
    let mut count = 0;
    'next_anshou: for anshou in 0..=999 {
        let d1 = digit1(anshou);
        let d2 = digit2(anshou);
        let d3 = digit3(anshou);

        let mut i = 0;
        let mut d1_found = false;
        while i < s.len() {
            if s[i] == d1 {
                d1_found = true;
                break;
            }
            i += 1;
        }

        if !d1_found {
            continue;
        }

        let mut j = i + 1;
        let mut d2_found = false;
        while j < s.len() {
            if s[j] == d2 {
                d2_found = true;
                break;
            }
            j += 1;
        }

        if !d2_found {
            continue;
        }

        let mut k = j + 1;
        let mut d3_found = false;
        while k < s.len() {
            if s[k] == d3 {
                d3_found = true;
                break;
            }
            k += 1;
        }

        if !d3_found {
            continue;
        }

        count += 1;
    }
    count
}
