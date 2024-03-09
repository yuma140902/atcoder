#[allow(unused_imports)]
use std::collections::*;
use std::rc::Rc;

use proconio::input;

#[derive(Debug)]
enum Query {
    Append(u64, u64),
    Remove(u64),
}

struct Node {
    value: u64,
    next: Option<Rc<Node>>,
    prev: Option<Rc<Node>>,
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize
    };
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        input! {
            ty: u32
        };
        if ty == 1 {
            input! {
                x: u64, y: u64
            };
            queries.push(Query::Append(x, y));
        } else {
            input! {
                x: u64
            };
            queries.push(Query::Remove(x));
        }
    }

    let mut linked_list = Node {
        value: *a.first().unwrap(),
        next: None,
        prev: None,
    };

    let mut map = HashMap::new();
    map.insert(*a.last().unwrap(), &linked_list);

    for ai in a.into_iter().rev().skip(1) {
        let node = Node {
            value: ai,
            next: Some(Box::new(linked_list)),
        };
        linked_list = node;
    }

    todo!();
}
