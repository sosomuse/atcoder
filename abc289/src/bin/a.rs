use proconio::input;

fn main() {
    input! {
        s: String,
    }

    for c in s.chars() {
        if c == '0' {
            print!("1");
        } else {
            print!("0");
        }
    }
}
