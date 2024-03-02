fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }

    let mut p = 0;
    let mut b = [0, 0, 0, 0];
    for ai in a.iter() {
        b[0] = 1;
        for k in (0..=3).rev() {
            let next_idx = k + ai;
            if next_idx >= 4 {
                p += b[k];
            } else {
                b[next_idx] = b[k];
            }
            b[k] = 0;
        }
    }
    println!("{}", p);
}
