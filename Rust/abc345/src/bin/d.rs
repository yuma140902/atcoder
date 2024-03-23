#[allow(unused_imports)]
use std::collections::*;

fn empty(map: &Vec<Vec<bool>>, x: i64, y: i64, w: i64, h: i64) -> bool {
    for xi in 0..w {
        for yi in 0..h {
            if x + xi < 0
                || y + yi < 0
                || x + xi >= map[0].len() as i64
                || y + yi >= map.len() as i64
            {
                return false;
            }
            if map[(y + yi) as usize][(x + xi) as usize] {
                return false;
            }
        }
    }
    true
}

fn count_empties(map: &Vec<Vec<bool>>) -> i64 {
    let mut count = 0;
    for row in map {
        for &x in row {
            if !x {
                count += 1;
            }
        }
    }
    count
}

fn place(map: &mut Vec<Vec<bool>>, x: i64, y: i64, w: i64, h: i64) {
    for xi in 0..w {
        for yi in 0..h {
            map[(y + yi) as usize][(x + xi) as usize] = true;
        }
    }
}

fn find_empty(map: &Vec<Vec<bool>>, h: usize, w: usize) -> (usize, usize) {
    for x in 0..w {
        for y in 0..h {
            if !map[y][x] {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn solve(
    map: &Vec<Vec<bool>>,
    tiles: &Vec<(i64, i64)>,
    h: usize,
    w: usize,
    memo: &mut HashSet<(Vec<Vec<bool>>, Vec<(i64, i64)>)>,
) -> bool {
    if memo.contains(&(map.clone(), tiles.clone())) {
        return false;
    }
    if map.iter().all(|row| row.iter().all(|&x| x)) {
        return true;
    }
    let empties = count_empties(&map);

    for i in 0..tiles.len() {
        let mut tiles_cpy = tiles.clone();
        tiles_cpy.remove(i);
        if empties < tiles[i].0 * tiles[i].1 {
            continue;
        }
        let (x, y) = find_empty(map, h, w);
        if empty(&map, x as i64, y as i64, tiles[i].0, tiles[i].1) {
            let mut map_cpy = map.clone();
            place(&mut map_cpy, x as i64, y as i64, tiles[i].0, tiles[i].1);
            if solve(&map_cpy, &tiles_cpy, h, w, memo) {
                return true;
            }
        }
        if empty(&map, x as i64, y as i64, tiles[i].1, tiles[i].0) {
            let mut map_cpy = map.clone();
            place(&mut map_cpy, x as i64, y as i64, tiles[i].1, tiles[i].0);
            if solve(&map_cpy, &tiles_cpy, h, w, memo) {
                return true;
            }
        }
    }
    memo.insert((map.clone(), tiles.clone()));
    false
}

#[argio::argio]
fn main(n: usize, h: usize, w: usize, ab: [(i64, i64); n]) -> &'static str {
    let map = vec![vec![false; w]; h];
    let mut memo = HashSet::new();
    if solve(&map, &ab, h, w, &mut memo) {
        "Yes"
    } else {
        "No"
    }
}

