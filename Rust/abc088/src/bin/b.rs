use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
enum Player {
    Alice,
    Bob,
}

fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }

    let mut heap = BinaryHeap::new();
    for ai in a {
        heap.push(ai);
    }

    let mut alice = 0;
    let mut bob = 0;
    let mut player = Player::Alice;

    while let Some(max) = heap.pop() {
        if player == Player::Alice {
            alice += max;
            player = Player::Bob;
        } else {
            bob += max;
            player = Player::Alice;
        }
    }

    let ans = alice - bob;
    println!("{}", ans);
}
