use itertools::Itertools;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        s: [String; h]
    }

    let mut field = vec![];
    for s_row in s {
        field.push(s_row.chars().collect_vec());
    }

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == '.' {
                let mut count = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let i = i as i32 + di;
                        let j = j as i32 + dj;
                        if 0 <= i
                            && i < field.len() as i32
                            && 0 <= j
                            && j < field[i as usize].len() as i32
                            && field[i as usize][j as usize] == '#'
                        {
                            count += 1;
                        }
                    }
                }
                print!("{}", count);
            } else {
                print!("#");
            }
        }
        println!();
    }
}
