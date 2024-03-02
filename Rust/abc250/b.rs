fn main() {
    proconio::input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut fa = -1;
    for i in 0..(a * n) {
        let mut fb = -1;
        if i % a == 0 {
            fa *= -1;
        }
        for j in 0..(b * n) {
            if j % b == 0 {
                fb *= -1;
            }
            print!("{}", if fa * fb == 1 { "." } else { "#" });
        }
        println!();
    }
}
