use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    };

    dbg!(s, t);
}
