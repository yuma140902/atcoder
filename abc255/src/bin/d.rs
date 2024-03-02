use std::iter::Sum;

fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        x: [i64; q]
    }

    //let sum_a: i64 = a.iter().sum();
    a.sort();
    let mut sums = vec![a[0]];
    for i in 1..n {
        let p: i64 = sums.iter().sum();
        sums[i] = p + a[i];
    }

    for xi in x.iter() {
        let mut i = 0;
        while a[i] < *xi {
            i += 1;
        }
        println!(
            "{}",
            (sums[i] - (*xi * i as i64))try!iexdiojweiofjwioefjiowej:() + (sums[n - 1] - *xi * (n - i) as i64).abs()
        );
    }
}
