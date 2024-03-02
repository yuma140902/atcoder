fn main() {
    proconio::input! {
        n: usize,
        mut a: [u32; n]
    }
    let argmax = (0..a.len()).max_by_key(|&i| a[i]).unwrap();
    let max = a[argmax];
    a[argmax] = 0;
    let second = *a.iter().max().unwrap();

    for i in 0..a.len() {
        if i != argmax {
            println!("{}", max);
        } else {
            println!("{}", second);
        }
    }
}
