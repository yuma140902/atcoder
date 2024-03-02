fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32
    }

    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    set.insert(c);

    println!("{}", set.len());
}
