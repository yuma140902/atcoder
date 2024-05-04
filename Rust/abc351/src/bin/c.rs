#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, a: [i64; n]) -> usize {
    let mut queue = VecDeque::new();

    for i in 0..n {
        queue.push_back(a[i]);
        while queue.len() >= 2 {
            if let (Some(first), Some(second)) = (queue.pop_back(), queue.pop_back()) {
                if first != second {
                    queue.push_back(second);
                    queue.push_back(first);
                    break;
                } else {
                    queue.push_back(first + 1);
                }
            }
        }
    }
    queue.len()
}
