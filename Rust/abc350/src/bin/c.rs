#[allow(unused_imports)]
use std::collections::*;

fn sort(a: &mut [i64], ops: &mut Vec<(usize, usize)>, index: &mut HashMap<i64, usize>) {
    for i in 1..=a.len() {
        let j = *index.get(&(i as i64)).unwrap();
        if i != j + 1 {
            ops.push((i - 1, j));
            a.swap(i - 1, j);
            index.insert(a[i - 1], i - 1);
            index.insert(a[j], j);
        }
    }
}

#[argio::argio]
fn main(n: usize, mut a: [i64; n]) {
    let mut ops = vec![];
    let mut index = HashMap::new();
    for i in 0..n {
        index.insert(a[i], i);
    }
    sort(&mut a, &mut ops, &mut index);

    println!("{}", ops.len());
    for op in &ops {
        if op.0 > op.1 {
            println!("{} {}", op.1 + 1, op.0 + 1);
        } else {
            println!("{} {}", op.0 + 1, op.1 + 1);
        }
    }

    #[cfg(debug_assertions)]
    {
        dbg!(&a);
        for i in 0..a.len() {
            assert_eq!(a[i], i as i64 + 1);
        }
        assert!(ops.len() <= n - 1);
    }
}
