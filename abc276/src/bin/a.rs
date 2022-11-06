use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: isize = -1;

    for i in 0..s.len() {
        if s[i] == 'a' {
            ans = i as isize + 1;
        }
    }

    println!("{}", ans);
}
