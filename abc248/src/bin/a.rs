use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let a = "1234567890";

    for v in a.chars() {
        if !s.contains(v) {
            println!("{}", v)
        }
    }
}
