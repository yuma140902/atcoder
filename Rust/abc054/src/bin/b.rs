#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;

fn get<T: Copy>(a: &Vec<Vec<T>>, x: usize, y: usize) -> Option<T> {
    a.get(x).map(|v| v.get(y)).flatten().map(|r| *r)
}

fn match_kernel<T: PartialEq + Copy>(
    image: &Vec<Vec<T>>,
    kernel: &Vec<Vec<T>>,
    x: usize,
    y: usize,
    ksize: usize,
) -> bool {
    for kx in 0..ksize {
        for ky in 0..ksize {
            if get(&image, x + kx, y + ky) != get(&kernel, kx, ky) {
                return false;
            }
        }
    }
    true
}

#[argio::argio]
fn main(n: usize, m: usize, a: [String; n], b: [String; m]) -> &'static str {
    let a = a.into_iter().map(|s| s.chars().collect_vec()).collect_vec();
    let b = b.into_iter().map(|s| s.chars().collect_vec()).collect_vec();

    for x in 0..n {
        for y in 0..n {
            if match_kernel(&a, &b, x, y, m) {
                return "Yes";
            }
        }
    }

    "No"
}
