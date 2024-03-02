fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        q: usize,
        bc: [(i32, i32); q]
    }

    let mut map = std::collections::HashMap::<i32, i32>::new();
    for &item in &a {
        map.insert(item, map.get(&item).unwrap_or(&0) + 1);
    }

    let mut sum = 0;
    for &item in &a {
        sum += item;
    }

    for (b, c) in bc {
        let &num = map.get(&b).unwrap_or(&0);
        sum += (c - b) * num;
        map.insert(c, map.get(&c).unwrap_or(&0) + num);
        map.insert(b, 0);
        println!("{}", sum);
    }
}
