#[allow(unused_imports)]
use std::collections::*;

fn is_triangle(p: (i64, i64), q: (i64, i64), r: (i64, i64)) -> bool {
    let (px, py) = p;
    let (qx, qy) = q;
    let (rx, ry) = r;
    (px - qx) * (py - ry) != (px - rx) * (py - qy)
}

#[argio::argio]
fn main(n: usize, points: [(i64, i64); n]) {
    let mut points = points.into_iter().collect::<BinaryHeap<_>>();

    while points.len() > 0 {
        let a = points.pop().unwrap();
        let b = points.pop().unwrap();
        let mut c = None;
        for d in points.iter() {
            if is_triangle(a, b, *d) {
                c = Some(d);
            }
        }
        if let Some(c) = c {
            points.remove(&c);
        }
    }
    todo!();
}
