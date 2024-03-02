fn main() {
    proconio::input! {
        n: u32
    }

    let modulo = 10007u32;

    if n == 1 || n == 2 {
        println!("0");
        return;
    }

    let mut prev2 = 0;
    let mut prev1 = 0;
    let mut prev0 = 1;

    for _ in 3u32..n {
        let new = (prev0 + prev1 + prev2) % modulo;
        prev2 = prev1;
        prev1 = prev0;
        prev0 = new;
    }

    println!("{}", prev0);
}
