fn main() {
    proconio::input! {
        s: String
    }

    let cs = s.as_bytes();
    let mut ans: Vec<char> = vec![];
    let mut ignore_next = false;
    for c in cs.iter().rev() {
        if *c == b'B' {
            ignore_next = true;
            continue;
        }
        if !ignore_next {
            ans.push(*c as char);
        }
        ignore_next = false;
    }
    ans.reverse();
    use std::iter::FromIterator;
    let ans = String::from_iter(ans.into_iter());
    println!("{}", ans);
}
