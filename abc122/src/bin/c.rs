use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    };

    let mut vec = vec![0; n];

    for i in 1..n {
        if s[i - 1] == 'A' && s[i] == 'C' {
            vec[i] = vec[i - 1] + 1;
        } else {
            vec[i] = vec[i - 1];
        }
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        println!("{}", vec[r - 1] - vec[l - 1]);
    }
}
