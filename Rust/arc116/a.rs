fn main() {
    proconio::input! {
        t: usize,
        cases: [u64; t]
    }

    for case in cases.iter() {
        println!(
            "{}",
            if case % 4 == 0 {
                "Even"
            } else if case % 2 == 0 {
                "Same"
            } else {
                "Odd"
            }
        );
    }
}
