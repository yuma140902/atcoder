fn main() {
    proconio::input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64
    }
    if d == 0 {
        println!("{}", (a - x).abs());
        return;
    }
    let m0 = (x - a) / d;
    dbg!(m0);
    let d0 = if m0 < 0 {
        abs_diff(a, x)
    } else if m0 > n - 1 {
        abs_diff(a + (n - 1) * d, x)
    } else {
        abs_diff(a + m0 * d, x)
    };
    dbg!(d0);

    let m1 = m0 + 1;
    let d1 = if m1 < 0 {
        abs_diff(a, x)
    } else if m1 > n - 1 {
        abs_diff(a + (n - 1) * d, x)
    } else {
        abs_diff(a + m1 * d, x)
    };
    dbg!(m1);
    dbg!(d1);
    let ans = d0.min(d1);
    println!("{}", ans);
}

fn abs_diff(a: i64, b: i64) -> u64 {
    if a >= 0 && b >= 0 {
        let a = a as u64;
        let b = b as u64;
        if a >= b {
            a - b
        } else {
            b - a
        }
    } else if a < 0 && b < 0 {
        abs_diff(-a, -b)
    } else if a < 0 {
        let a = (-a) as u64;
        let b = b as u64;
        a + b
    } else {
        abs_diff(-a, -b)
    }
}
