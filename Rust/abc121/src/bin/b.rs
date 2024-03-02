use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n],
    }

    let ans = a
        .iter()
        .map(|features| {
            features
                .iter()
                .zip(b.iter())
                .map(|(ai, bi)| ai * bi)
                .sum::<i32>()
        })
        .filter(|sum| *sum + c > 0)
        .count();

    println!("{}", ans);
}
