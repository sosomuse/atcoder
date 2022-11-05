use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut p = vec![];
    let mut count = 0;

    for i in 0..n {
        if s[i] == 'I' {
            count += 1;
        } else {
            count -= 1;
        }
        p.push(count);
    }
}
