#[argio::argio]
fn main(a: u64, b: u64, c: u64) -> &'static str {
    for n in 1..=b {
        if a * n % b == c {
            return "YES";
        }
    }

    "NO"
}
