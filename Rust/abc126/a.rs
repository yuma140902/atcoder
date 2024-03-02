fn main() {
    proconio::input! {
        _n: i32,
        k: i32,
        s: String
    }

    let mut s = s.into_bytes();
    s[(k - 1) as usize] += 32;
    let result;
    unsafe {
        result = String::from_utf8_unchecked(s);
    }
    println!("{}", result);
}
