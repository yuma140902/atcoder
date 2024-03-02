fn main() {
    proconio::input! {
        n: i32,
        l: i32
    }

    let j = clamp(1 - l, 1, n);
    let ans = n * (l - 1) + n * (n + 1) / 2 - (l + j - 1);

    println!("{}", ans);
}

fn clamp(num: i32, min: i32, max: i32) -> i32 {
    if num < min {
        min
    } else if max < num {
        max
    } else {
        num
    }
}
