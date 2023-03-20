use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 3;
    if s[0] == s[1] && s[1] == s[2] {
        ans = 1;
    } else if s[0] != s[1] && s[1] != s[2] && s[0] != s[2] {
        ans = 6;
    }

    println!("{}", ans);
}
