use itertools::Itertools;

#[argio::argio]
fn main(n: usize, txy: [(i64, i64, i64); n]) -> String {
    let txy = Some((0, 0, 0))
        .into_iter()
        .chain(txy.into_iter())
        .collect_vec();

    for i in 0..(txy.len() - 1) {
        let (t1, x1, y1) = txy[i];
        let (t2, x2, y2) = txy[i + 1];

        let dt = t2 - t1;
        let distance = (x2 - x1).abs() + (y2 - y1).abs();
        if !(dt >= distance && (dt - distance) % 2 == 0) {
            return "No".to_string();
        }
    }
    return "Yes".to_string();
}
