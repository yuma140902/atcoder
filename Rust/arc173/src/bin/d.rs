#[allow(unused_imports)]
use std::collections::*;

mod table_display {
    pub struct TableDisplay<'a, T>(pub &'a Vec<Vec<T>>);
    impl<T: std::fmt::Display> std::fmt::Display for TableDisplay<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for x in 0..self.0.len() {
                for y in 0..self.0[x].len() {
                    write!(f, "{} ", self.0[x][y])?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}

#[argio::argio]
fn main(n: usize, m: usize, edges: [(usize, usize, char); m]) -> &'static str {
    let mut forward_map = HashMap::<(usize, char), HashSet<usize>>::new();
    let mut backward_map = HashMap::<(usize, char), HashSet<usize>>::new();
    for v in 1..=n {
        forward_map.insert((v, '('), HashSet::new());
        forward_map.insert((v, ')'), HashSet::new());
        backward_map.insert((v, '('), HashSet::new());
        backward_map.insert((v, ')'), HashSet::new());
    }
    for (from, to, c) in edges {
        forward_map.get_mut(&(from, c)).unwrap().insert(to);
        backward_map.get_mut(&(to, c)).unwrap().insert(from);
    }
    let forward_map = forward_map;
    let backward_map = backward_map;

    //eprintln!("{}", table_display::TableDisplay(&edge_table));

    let mut right_walks = HashSet::<(usize, usize)>::new();
    for from in 1..=n {
        for mid in &forward_map[&(from, '(')] {
            for to in &forward_map[&(*mid, ')')] {
                right_walks.insert((from, *to));
            }
        }
    }
    if right_walks.iter().any(|(from, to)| *from == *to) {
        return "Yes";
    }

    loop {
        let mut new = HashSet::<(usize, usize)>::new();
        for (from, to) in &right_walks {
            for fromfrom in &backward_map[&(*from, '(')] {
                for toto in &forward_map[&(*to, ')')] {
                    new.insert((*fromfrom, *toto));
                }
            }
        }
        let old_len = right_walks.len();
        right_walks.extend(new);
        let new_len = right_walks.len();
        if old_len == new_len {
            break;
        }
    }
    if right_walks.iter().any(|(from, to)| *from == *to) {
        return "Yes";
    }

    let mut right_walks_forward_map = HashMap::<usize, HashSet<usize>>::new();
    for v in 1..=n {
        right_walks_forward_map.insert(v, HashSet::new());
    }
    for (from, to) in &right_walks {
        right_walks_forward_map.get_mut(from).unwrap().insert(*to);
    }
    let right_walks_forward_map = right_walks_forward_map;

    for start in 1..=n {
        let mut intermediates = VecDeque::<usize>::new();
        intermediates.push_back(start);
        while let Some(intermediate) = intermediates.pop_front() {
            for next in &right_walks_forward_map[&intermediate] {
                intermediates.push_back(*next);
                if *next == start {
                    return "Yes";
                }
            }
        }
    }

    "No"
}
