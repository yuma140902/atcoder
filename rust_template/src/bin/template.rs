#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::nursery)]

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use utils::*;

fn main() {
    input! {}
}

mod utils {
    #[allow(dead_code)]
    const INF: i32 = i32::MAX - i32::MAX / 8;
    #[allow(dead_code)]
    const INFL: i64 = i64::MAX - i64::MAX / 8;
    #[allow(dead_code)]
    const EPS: f64 = 1e-10;

    #[allow(dead_code)]
    pub fn fequal(f: f64, g: f64) -> bool {
        (f - g).abs() < EPS
    }

    #[allow(dead_code)]
    pub fn assign_max<T: PartialOrd>(lhs: &mut T, rhs: T) -> bool {
        if *lhs < rhs {
            *lhs = rhs;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn assign_min<T: PartialOrd>(lhs: &mut T, rhs: T) -> bool {
        if *lhs > rhs {
            *lhs = rhs;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    const AROUND: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub struct TableDisplay<'a, T>(&'a Vec<Vec<T>>);

    impl<T: std::fmt::Display> std::fmt::Display for TableDisplay<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for x in 0..self.0.len() {
                for y in 0..self.0[x].len() {
                    write!(f, "{} ", self.0[x][y])?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
