#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, ps: [(i64, i64); n]) {
    for &(px, py) in &ps {
        let mut max_dist = i64::MIN;
        let mut max_i = -1;
        for (i, &(qx, qy)) in ps.iter().enumerate() {
            let dist = (px - qx).pow(2) + (py - qy).pow(2);
            if dist > max_dist {
                max_dist = dist;
                max_i = i as i64 + 1;
            }
        }
        println!("{}", max_i);
    }
}
