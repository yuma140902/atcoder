fn main() {
    proconio::input! {
        n: i32,
        k: i32,
        x: i32,
        y: i32
    }

    let ans = if n <= k { n * x } else { k * x + (n - k) * y };

    println!("{}", ans);
}
