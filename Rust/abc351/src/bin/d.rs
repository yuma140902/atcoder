#[allow(unused_imports)]
use std::collections::*;

use itertools::Itertools;
use proconio::marker::Chars;

fn hash_around(s: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let positions = [
        (x as i64 - 1, y as i64),
        (x as i64 + 1, y as i64),
        (x as i64, y as i64 - 1),
        (x as i64, y as i64 + 1),
    ];

    for (x, y) in positions.into_iter() {
        if x < 0 || y < 0 {
            continue;
        }
        if let Some(row) = s.get(y as usize) {
            if let Some(cell) = row.get(x as usize) {
                if *cell == '#' {
                    return true;
                }
            }
        }
    }
    false
}

fn id(x: usize, y: usize, w: usize) -> usize {
    y * w + x
}

fn dfs(s: i64, v: i64, used: &mut Vec<i64>, count: &mut i64, e: &Vec<Vec<i64>>, unique: i64) {
    if used[v as usize] == unique {
        return;
    }
    used[v as usize] = unique;
    *count += 1;
    let sz = e[v as usize].len();
    for i in 0..sz {
        dfs(s, e[v as usize][i], used, count, e, unique);
    }
}

#[argio::argio]
fn main(h: usize, w: usize, s: [Chars; h]) -> i64 {
    const DX: [i64; 4] = [0, 0, 1, -1];
    const DY: [i64; 4] = [1, -1, 0, 0];

    let mut e: Vec<Vec<i64>> = vec![vec![]; h * w];

    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                continue;
            }
            for k in 0..4 {
                let xx = x as i64 + DY[k];
                let yy = y as i64 + DX[k];
                if (0..h as i64).contains(&yy) && (0..w as i64).contains(&xx) {
                    e[id(x, y, w)].push(id(xx as usize, yy as usize, w) as i64);
                }
            }

            if hash_around(&s, x, y) {
                e[id(x, y, w)].clear();
                continue;
            }
        }
    }

    let mut used = vec![-1; h * w];
    let mut count;
    let mut ans = 0;

    let mut unique = 42;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '.' && used[id(x, y, w)] < 0 {
                count = 0;
                dfs(
                    id(x, y, w) as i64,
                    id(x, y, w) as i64,
                    &mut used,
                    &mut count,
                    &e,
                    unique,
                );
                ans = ans.max(count);
            }
            unique += 10;
        }
    }

    ans
}
