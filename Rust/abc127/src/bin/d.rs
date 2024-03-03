#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, m: usize, a: [u64; n], bc: [(u64, u64); m]) -> u64 {
    let mut cb = bc
        .into_iter()
        .map(|(b, c)| (c, b))
        .collect::<BinaryHeap<_>>();
    for ai in &a {
        cb.push((*ai, 1));
    }

    let max_num_cards = a.len() as u64;
    let mut num_cards = 0;
    let mut sum_cards = 0;

    while let Some((c, b)) = cb.pop() {
        let b = b.clamp(0, max_num_cards - num_cards);
        num_cards += b;
        sum_cards += b * c;
        if num_cards >= max_num_cards {
            break;
        }
    }

    sum_cards
}
