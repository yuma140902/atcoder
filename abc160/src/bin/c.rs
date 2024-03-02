fn main() {
    proconio::input! {
        k: u64,
        n: usize,
        a: [u64; n]
    }

    let mut b = vec![0; n];

    for i in 1..a.len() {
        b[i] = a[i] - a[i - 1];
    }
    b[0] = a[0] + k - a[a.len() - 1];

    let max_gap = b.iter().max().unwrap();
    let ans = k - max_gap;

    println!("{}", ans);
}
