fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        xs: [i32; n]
    }

    let ans = xs.iter().map(|&x| x.min(k - x) * 2).sum::<i32>();

    println!("{}", ans);
}
