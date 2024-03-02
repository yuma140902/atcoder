fn main() {
    proconio::input! {
        h: u128,
        w: u128
    }

    if h == 1 || w == 1 {
        println!("{}", 1);
        return;
    }

    let ans = ((w + 1) / 2) * ((h + 1) / 2) + (w / 2) * (h / 2);

    println!("{}", ans);
}
