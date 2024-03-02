fn main() {
    proconio::input! {
        s: String
    }

    let s = s.as_bytes();
    let mut cnt = 0;
    for i in 1usize..s.len() {
        if s[i - 1] != s[i] {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
