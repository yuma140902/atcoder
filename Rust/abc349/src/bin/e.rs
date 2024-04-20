#[allow(unused_imports)]
use std::collections::*;
use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Red,
    Blue,
}

type Points = [i64; 9];
type Board = [Option<Player>; 9];
type FullBoard = [Player; 9];

const AXIS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn check_bingo(board: &Board) -> Option<Player> {
    for a in AXIS {
        if a.into_iter().map(|i| board[i]).all_equal() && board[a[0]].is_some() {
            return board[a[0]];
        }
    }
    None
}

fn check_full(board: &Board) -> Option<FullBoard> {
    let mut full = [Player::Red; 9];
    for i in 0..9 {
        if let Some(p) = board[i] {
            full[i] = p;
        } else {
            return None;
        }
    }
    Some(full)
}

fn get_winner(board: &FullBoard, points: &Points) -> Player {
    let mut red = 0;
    let mut blue = 0;

    for i in 0..9 {
        match board[i] {
            Player::Red => red += points[i],
            Player::Blue => blue += points[i],
        }
    }

    if red > blue {
        Player::Red
    } else {
        Player::Blue
    }
}

fn next_board(board: &Board, i: usize, player: Player) -> Board {
    let mut b = *board;
    b[i] = Some(player);
    b
}

fn find_winner(
    board: &Board,
    points: &Points,
    player: Player,
    memo: &mut HashMap<Board, Player>,
) -> Player {
    if let Some(w) = memo.get(board) {
        return *w;
    }
    if let Some(w) = check_bingo(board) {
        memo.insert(*board, w);
        return w;
    }
    if let Some(full) = check_full(board) {
        let w = get_winner(&full, points);
        memo.insert(*board, w);
        return w;
    }

    if (0..9)
        .filter(|&i| board[i].is_none())
        .map(|i| next_board(board, i, player))
        .map(|b| find_winner(&b, points, player.opponent(), memo))
        .any(|winner| winner == player)
    {
        memo.insert(*board, player);
        player
    } else {
        memo.insert(*board, player.opponent());
        player.opponent()
    }
}

#[argio::argio]
fn main(points: [i64; 9]) -> Player {
    let points: Points = points.try_into().unwrap();
    let board: Board = [None; 9];
    let mut memo = HashMap::<Board, Player>::new();

    find_winner(&board, &points, Player::Red, &mut memo)
}

impl Player {
    fn opponent(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::Red => "Takahashi",
                Player::Blue => "Aoki",
            }
        )
    }
}
