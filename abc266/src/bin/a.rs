use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    println!("{}", s[(s.len() / 2 + s.len() % 2) - 1]);
}
