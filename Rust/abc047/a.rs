fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32
    }

    let ans: bool = a + b == c || b + c == a || c + a == b;
    println!("{}", if ans { "Yes" } else { "No" });
}
