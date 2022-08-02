use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    };

    for i in 0..l - 1 {
        print!("{}", s[i]);
    }

    for i in (l - 1..r).rev() {
        print!("{}", s[i]);
    }

    for i in r..s.len() {
        print!("{}", s[i]);
    }
}
