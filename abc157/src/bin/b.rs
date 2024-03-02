const IDXS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 4, 8],
    [2, 4, 6],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
];

fn check_bingo(holes: &[bool; 9]) -> bool {
    IDXS.iter()
        .any(|axis| axis.iter().all(|&index| holes[index]))
}

fn main() {
    proconio::input! {
        a: [u32; 9],
        n: u32,
        b: [u32; n],
    }

    let mut holes = [false; 9];

    for bi in b {
        for (i, &ai) in a.iter().enumerate() {
            if ai == bi {
                holes[i] = true;
            }
        }
    }
    if check_bingo(&holes) {
        println!("Yes");
    } else {
        println!("No");
    }
}
