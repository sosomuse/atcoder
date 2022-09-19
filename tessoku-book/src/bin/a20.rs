use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
