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
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q]
    }

    for li in l {
        let li = li - 1;
        debug!(li);
        if a[li] == n {
            continue;
        }
        if a.contains(&(a[li] + 1)) {
            continue;
        }
        a[li] += 1;
    }

    println!("{}", itertools::join(a.iter(), " "));
}
