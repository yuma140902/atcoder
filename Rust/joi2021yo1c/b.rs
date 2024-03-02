fn main() {
    proconio::input! {
      n: usize,
      s: String
    }

    let s = s.as_bytes();
    let mut cnt = 0;
    for i in 0usize..n {
        if i % 2 == 0 && s[i] != b'I' {
            cnt += 1;
        }
        if i % 2 == 1 && s[i] != b'O' {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
