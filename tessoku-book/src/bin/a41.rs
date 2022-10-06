use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = false;

    for i in 0..n - 2 {
        if s[i] == 'R' && s[i + 1] == 'R' && s[i + 2] == 'R' {
            ans = true;
        }

        if s[i] == 'B' && s[i + 1] == 'B' && s[i + 2] == 'B' {
            ans = true;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
