fn main() {
    proconio::input! {
        n: u32,
        d: u32
    }
    let m = 2 * d + 1;
    let ans = (n + m - 1) / m;
    println!("{}", ans);
}
