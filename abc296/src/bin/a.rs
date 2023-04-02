use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut is_alternating = true;

    for i in 1..n {
        if s[i] == s[i - 1] {
            is_alternating = false;
            break;
        }
    }

    println!("{}", if is_alternating { "Yes" } else { "No" });
}
