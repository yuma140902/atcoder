#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;
use proconio::marker::Chars;

fn remove_rows(map: ndarray::Array2<char>) -> ndarray::Array2<char> {
    let mut v: Vec<char> = vec![];
    let mut num_rows = 0;
    let w = map.ncols();
    for row in map.rows() {
        if row.iter().any(|c| *c == '#') {
            v.extend(&mut row.into_iter());
            num_rows += 1;
        }
    }

    ndarray::Array2::from_shape_vec((num_rows, w), v).unwrap()
}

#[argio::argio]
fn main(h: usize, w: usize, map: [Chars; h]) {
    let map = map.into_iter().flatten().collect_vec();
    let map: ndarray::Array2<char> = ndarray::Array::from_shape_vec((h, w), map).unwrap();

    let map = remove_rows(map);
    let map = map.reversed_axes();
    let map = remove_rows(map);
    let map = map.t();
    for row in map.rows() {
        for item in row.into_iter() {
            print!("{}", item);
        }
        println!();
    }
}
