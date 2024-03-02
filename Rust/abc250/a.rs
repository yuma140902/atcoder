fn main() {
    proconio::input! {
        h: i32,
        w: i32,
        r: i32,
        c: i32,
    }

    let conds = [r - 1 >= 1, c - 1 >= 1, r + 1 <= h, c + 1 <= w];
    let count = conds.iter().filter(|b| **b).count();
    println!("{}", count);
}
