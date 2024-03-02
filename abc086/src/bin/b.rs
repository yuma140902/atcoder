fn digits(num: i32) -> u32 {
    (num as f32).log10().floor() as u32 + 1
}

fn main() {
    proconio::input! {
        a: i32,
        b: i32
    }

    let n = a * 10_i32.pow(digits(b)) + b;

    let sqrt = (n as f32).sqrt().floor() as i32;

    if n == sqrt * sqrt {
        println!("Yes");
    } else {
        println!("No");
    }
}
