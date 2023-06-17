use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    for i in 0..n {
        let v = s[i];

        print!("{}{}", v, v)
    }
}
