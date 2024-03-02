fn main() {
    proconio::input! {
        n: i32,
        k: i32,
        ds: [i32; k]
    }

    let mut ans = n;
    while !check(ans, &ds) {
        ans += 1;
    }
    println!("{}", ans);
}

fn check(mut candid: i32, dislikes: &Vec<i32>) -> bool {
    while candid != 0 {
        if dislikes.contains(&(candid % 10)) {
            return false;
        }
        candid /= 10;
    }
    true
}
