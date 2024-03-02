fn main() {
    proconio::input! {
        w: String
    }

    use std::collections::HashMap;
    let mut map: HashMap<u8, i32> = HashMap::new();
    for c in w.as_bytes().iter() {
        map.insert(*c, map.get(c).unwrap_or(&0) + 1);
    }

    let mut ans = true;
    for (_, v) in map.iter() {
        if v % 2 != 0 {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
