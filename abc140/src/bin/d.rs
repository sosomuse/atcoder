use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let mut ans = 0;
    for i in 0..n {
        if s[i] == 'L' && i > 0 && s[i - 1] == 'L' {
            ans += 1;
        }

        if s[i] == 'R' && i < n - 1 && s[i + 1] == 'R' {
            ans += 1;
        }
    }

    ans += k * 2;
    println!("{}", ans.min(n - 1));
}
