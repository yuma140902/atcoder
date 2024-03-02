fn main() {
    proconio::input! {
        n: usize,
        mat: [(u32, u32); n]
    }

    let mut prev = mat[0usize];

    for i in 1usize..n {
        let scale = u32::max(div_ceil(prev.0, mat[i].0), div_ceil(prev.1, mat[i].1));
        prev = mult(&mat[i], scale);
    }

    println!("{}", prev.0 + prev.1);
}

fn div_ceil(prev: u32, maybe_current: u32) -> u32 {
    if prev % maybe_current == 0 {
        prev / maybe_current
    } else {
        prev / maybe_current + 1
    }
}

fn mult(ratio: &(u32, u32), scale: u32) -> (u32, u32) {
    (ratio.0 * scale, ratio.1 * scale)
}
/*
fn check(prev: &(u32, u32), maybe_current: &(u32, u32)) -> bool {
    maybe_current.0 >= prev.0 && maybe_current.1 >= prev.1
}*/
