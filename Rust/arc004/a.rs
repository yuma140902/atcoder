fn main() {
    proconio::input! {
        n: usize,
        ps: [(i32, i32); n]
    }

    let mut max_dist = 0;
    for i in 0..n - 1 {
        for j in i..n {
            let dist = (ps[i].0 - ps[j].0).pow(2) + (ps[i].1 - ps[j].1).pow(2);
            max_dist = max_dist.max(dist);
        }
    }

    println!("{}", (max_dist as f32).sqrt());
}
