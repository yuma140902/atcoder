#[argio::argio]
fn main(n: u64) -> u64 {
    n * (n - 1) / 2
}
