use ascii::AsAsciiStr;

fn main() {
    proconio::input! {
        s: String
    }

    let mut prev = '\0';
    for c in s.chars() {
        if prev == c {
            println!("Bad");
            return;
        }
        prev = c;
    }
    println!("Good");
}
