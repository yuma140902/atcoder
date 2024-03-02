fn main() {
    proconio::input! {
        h: [i32; 3],
        w: [i32; 3]
    }

    let h = [h[0], h[1], h[2]];
    let w = [w[0], w[1], w[2]];
    let ans = dfs(h, w);
    println!("{}", ans);
}

fn dfs(h: [i32; 3], w: [i32; 3]) -> u32 {
    if h[0] <= 3 || h[1] <= 3 || h[2] <= 3 {
        return 0;
    }
    if w[0] <= 3 || w[1] <= 3 || w[2] <= 3 {
        return 0;
    }

    let mut sum = 0;
    for hi in 0..3 {
        for wi in 0..3 {
            for i in 1..=h[hi].min(w[wi]) - 2 {
                sum +=
                    1 + {
                        let mut h = h;
                        h[hi] -= i as i32;
                        dfs(h, w)
                    } + {
                        let mut w = w;
                        w[hi] -= i as i32;
                        dfs(h, w)
                    };
            }
        }
    }
    return sum;
}
