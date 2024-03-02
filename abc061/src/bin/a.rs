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
        s: String
    }

    println!("{}", debug!(s));
}
