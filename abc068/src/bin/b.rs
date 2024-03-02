fn main() {
    proconio::input! {
        n: u32
    }

    let mut i = 1;

    while i <= n {
        i *= 2;
    }

    i /= 2;

    println!("{}", i);
}
