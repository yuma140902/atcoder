fn main() {
    proconio::input! {
        s: String
    }

    let (first, second) = s.split_at(2);
    let first: i32 = first.parse().unwrap();
    let second: i32 = second.parse().unwrap();

    let result = if (first <= 0 || 12 < first) && (second <= 0 || 12 < second) {
        "NA"
    } else if first <= 0 || 12 < first {
        "YYMM"
    } else if second <= 0 || 12 < second {
        "MMYY"
    } else {
        "AMBIGUOUS"
    };

    println!("{}", result);
}
