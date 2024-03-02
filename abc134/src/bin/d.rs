fn main() {
    proconio::input! {
        n: usize,
    }
    let a = {
        let mut a: Vec<u32> = Vec::with_capacity(n + 1);
        a.resize(n + 1, 0);
        for i in 1..=n {
            proconio::input! {
                num: u32
            }
            a[i] = num;
        }
        a
    };
    let mut b: Vec<u32> = Vec::with_capacity(n + 1);
    b.resize(n + 1, 0);

    for i in (1..=n).rev() {
        let mut m = 2;
        let mut sum = 0;
        while i * m <= n {
            sum += b[i * m];
            m += 1;
        }
        b[i] = if sum % 2 == a[i] { 0 } else { 1 }
    }

    let result = b
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i);
    println!("{}", result.clone().count());
    for i in result {
        println!("{}", i);
    }
}
