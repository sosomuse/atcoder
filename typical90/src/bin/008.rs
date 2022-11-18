use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![0; 8]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..=7 {
            dp[i + 1][j] += dp[i][j];

            if s[i] == 'a' && j == 0 {
                dp[i + 1][j + 1] += dp[i][j]
            };
            if s[i] == 't' && j == 1 {
                dp[i + 1][j + 1] += dp[i][j]
            };
            if s[i] == 'c' && j == 2 {
                dp[i + 1][j + 1] += dp[i][j]
            };
            if s[i] == 'o' && j == 3 {
                dp[i + 1][j + 1] += dp[i][j]
            };
            if s[i] == 'd' && j == 4 {
                dp[i + 1][j + 1] += dp[i][j]
            };
            if s[i] == 'e' && j == 5 {
                dp[i + 1][j + 1] += dp[i][j]
            };
            if s[i] == 'r' && j == 6 {
                dp[i + 1][j + 1] += dp[i][j]
            };
        }

        for j in 0..=7 {
            dp[i + 1][j] %= 1_000_000_007;
        }
    }

    println!("{}", dp[n][7]);
}
