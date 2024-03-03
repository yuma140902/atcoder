use itertools::Itertools;

fn solve_partial(r: usize, c: usize, field: &Vec<Vec<char>>, dp: &Vec<Vec<u64>>) -> u64 {
    let up = if r == 0 {
        u64::MAX
    } else {
        dp[r - 1][c]
            + if field[r][c] == '#' && field[r - 1][c] == '#' {
                0
            } else if field[r][c] == '#' {
                1
            } else {
                0
            }
    };
    let left = if c == 0 {
        u64::MAX
    } else {
        dp[r][c - 1]
            + if field[r][c] == '#' && field[r][c - 1] == '#' {
                0
            } else if field[r][c] == '#' {
                1
            } else {
                0
            }
    };

    up.min(left)
}

#[argio::argio]
fn main(h: usize, w: usize, s: [String; h]) -> u64 {
    let field = s.into_iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut dp = vec![vec![u64::MAX; w]; h];

    dp[0][0] = if field[0][0] == '#' { 1 } else { 0 };
    for distance in 1..(h + w) {
        for r in 0..=distance {
            let c = distance - r;
            if r < h && c < w {
                dp[r][c] = dp[r][c].min(solve_partial(r, c, &field, &dp));
            }
        }
    }

    /*
    for r in 0..h {
        for c in 0..w {
            let v = dp[r][c];
            eprint!("{} ", v);
        }
        eprintln!();
    }
    */

    dp[h - 1][w - 1]
}
