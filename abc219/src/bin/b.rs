use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [String; 3],
        t: Chars,
    };

    for c in t {
        let i = c.to_digit(10).unwrap() as usize - 1;
        print!("{}", s[i]);
    }
}
