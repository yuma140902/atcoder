fn main() {
    proconio::input! {
        w: i32,
        h: i32,
        n: usize,
        matrix: [(i32, i32, i32); n]
    }

    let mut begin = Point { x: 0, y: 0 };
    let mut end = Point { x: w, y: h };

    for (x, y, a) in matrix {
        if a == 1 && begin.x < x {
            begin.x = x;
        }
        if a == 2 && end.x > x {
            end.x = x;
        }
        if a == 3 && begin.y < y {
            begin.y = y;
        }
        if a == 4 && end.y > y {
            end.y = y;
        }
    }

    let ans = i32::max(end.x - begin.x, 0) * i32::max(end.y - begin.y, 0);
    println!("{}", ans);
}

struct Point {
    x: i32,
    y: i32,
}
