use num::integer;

type unum = u128;

fn main() {
    proconio::input! {
        a: unum,
        b: unum,
        c: unum,
        d: unum
    }

    let ans = solve(b, c, d) - solve(a - 1, c, d);

    println!("{}", ans);
}

fn solve(n: unum, d1: unum, d2: unum) -> unum {
    solve2(n, d1) + solve2(n, d2) - solve2(n, integer::lcm(d1, d2))
}

fn solve2(n: unum, d: unum) -> unum {
    n - n / d
}
