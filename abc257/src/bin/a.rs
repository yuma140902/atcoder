#[allow(unused_imports)]
use proconio::input;

#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! debug (($arg:expr) => (dbg!($arg)));

#[allow(unused_macros)]
#[cfg(not(debug_assertions))]
macro_rules! debug (($arg:expr) => ($arg));

fn main() {
    input! {
        n: i32,
        x: i32,
    }

    let i = (x - 1) / n;
    let c = i + 65;
    let c = c as u8 as char;

    println!("{}", c);
}
