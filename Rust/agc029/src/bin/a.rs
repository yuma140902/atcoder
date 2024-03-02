fn triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[argio::argio]
fn main(s: String) -> u64 {
    let mut ans = 0;
    let mut num_w = 0;

    for (i, c) in s.chars().enumerate() {
        if c == 'W' {
            ans += i as u64;
            num_w += 1;
        }
    }

    ans - triangle(num_w - 1)
}
