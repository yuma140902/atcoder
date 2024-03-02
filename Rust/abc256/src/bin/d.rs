use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        spans: [(i32, i32); n]
    }

    let bs = spans.iter().map(|(b, _)| *b).sorted().collect::<Vec<_>>();
    let es = spans.iter().map(|(_, e)| *e).sorted().collect::<Vec<_>>();

    let mut v = vec![];

    let mut bi = 0;
    let mut ei = 0;
    let mut idx = 0;
    let mut depth = 0;
    while idx <= es[n - 1] {
        while Some(&idx) == bs.get(bi) {
            if depth == 0 {
                v.push(idx);
            }
            depth += 1;
            bi += 1;
        }
        while Some(&idx) == es.get(ei) {
            depth -= 1;
            ei += 1;
            if depth == 0 {
                v.push(idx);
            }
        }
        idx += 1;
    }

    for i in v.iter() {
        println!("{}", i);
    }
}
