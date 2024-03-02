fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(i64, i64); n]
    }
    let mut max_distance = 0;
    for (i, (x, y)) in xy.iter().enumerate() {
        let has_light = a.contains(&(i + 1));
        let mut min_distance = std::i64::MAX;
        if !has_light {
            for j in a.iter() {
                let j = j - 1;
                let (lx, ly) = xy[j];
                let distance = (lx - x) * (lx - x) + (ly - y) * (ly - y);
                min_distance = distance.min(min_distance);
            }
        } else {
            min_distance = 0;
        }
        max_distance = max_distance.max(min_distance);
    }
    let max_distance = max_distance as f64;
    let r = max_distance.sqrt();
    println!("{}", r);
}
