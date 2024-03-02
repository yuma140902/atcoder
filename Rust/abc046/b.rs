fn main() {
    proconio::input! {
        n: u32,
        k: u32
    }

    println!("{}", k * (k - 1u32).pow(n - 1u32));
}
