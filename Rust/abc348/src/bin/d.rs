#[allow(unused_imports)]
use std::collections::*;

use proconio::marker::Chars;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MoveOrEnergy {
    Move,
    Energy,
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
    energies: Vec<(usize, usize, i64)>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Map {
    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Wall => print!("#"),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    energy_indices: Vec<usize>,
    x: usize,
    y: usize,
    prev_x: usize,
    prev_y: usize,
    energy: i64,
    move_or_energy: MoveOrEnergy,
}

#[argio::argio]
fn main(h: usize, w: usize, a: [Chars; h], n: usize, rce: [(usize, usize, i64); n]) -> String {
    let mut map = Map {
        cells: vec![vec![Cell::Empty; w]; h],
        energies: rce,
        start: (0, 0),
        goal: (0, 0),
    };

    let mut dp = vec![vec![i64::MIN; w]; h];

    for x in 0..w {
        for y in 0..h {
            if a[y][x] == 'S' {
                map.start = (x, y);
            } else if a[y][x] == 'T' {
                map.goal = (x, y);
            } else if a[y][x] == '#' {
                map.cells[y][x] = Cell::Wall;
            } else if a[y][x] == '.' {
                map.cells[y][x] = Cell::Empty
            } else {
                panic!();
            }
        }
    }

    let map = map;

    //map.print();

    let mut queue = VecDeque::new();
    queue.push_back(State {
        energy_indices: (0..n).collect(),
        energy: 0,
        x: map.start.0,
        y: map.start.1,
        prev_x: map.start.0,
        prev_y: map.start.1,
        move_or_energy: MoveOrEnergy::Move,
    });

    while let Some(current) = queue.pop_front() {
        if current.x == map.goal.0 && current.y == map.goal.1 {
            return "Yes".to_string();
        }

        if dp[current.y][current.x] >= current.energy {
            continue;
        }
        dp[current.y][current.x] = current.energy;

        let mut used_energy = false;

        for &ei in &current.energy_indices {
            let (y, x, e) = map.energies[ei];
            if x - 1 == current.x && y - 1 == current.y && e > current.energy {
                let next_energy_indices = current
                    .energy_indices
                    .iter()
                    .filter(|&&i| i != ei)
                    .copied()
                    .collect();
                let next = State {
                    energy_indices: next_energy_indices,
                    energy: e,
                    prev_x: current.prev_x,
                    prev_y: current.prev_y,
                    x: current.x,
                    y: current.y,
                    move_or_energy: MoveOrEnergy::Energy,
                };
                used_energy = true;
                queue.push_back(next);
            }
        }

        if used_energy {
            continue;
        }

        if current.energy > 0 {
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = current.x as i64 + dx;
                let ny = current.y as i64 + dy;
                if nx < 0
                    || nx >= w as i64
                    || ny < 0
                    || ny >= h as i64
                    || (current.move_or_energy == MoveOrEnergy::Move
                        && nx == current.prev_x as i64
                        && ny == current.prev_y as i64)
                {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if map.cells[ny][nx] == Cell::Wall {
                    continue;
                }
                let next = State {
                    energy_indices: current.energy_indices.clone(),
                    energy: current.energy - 1,
                    prev_x: current.x,
                    prev_y: current.y,
                    x: nx,
                    y: ny,
                    move_or_energy: MoveOrEnergy::Move,
                };
                queue.push_back(next);
            }
        }
    }

    if dp[map.goal.1][map.goal.0] >= 0 {
        return "Yes".to_string();
    }
    "No".to_string()
}
