fn next(a: &Vec<u32>, b: &Vec<bool>, i: usize) -> Option<usize> {
    for k in i..a.len() {
        if b[k] && a[i] < a[k] {
            return Some(k);
        }
    }
    None
}

fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    let mut b: Vec<bool> = Vec::with_capacity(n);
    b.resize(n, true);

    let mut count = 0;

    for i in 0..n {
        if !b[i] {
            continue;
        }
        b[i] = false;
        let mut index = Some(i);
        while {
            index = next(&a, &b, index.unwrap());
            index.is_some()
        } {
            b[index.unwrap()] = false;
        }
        count += 1;
    }

    println!("{}", count);
}
