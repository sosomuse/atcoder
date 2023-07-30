use proconio::input;
use proconio::marker::Chars;

const MOD: isize = 998244353;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut dp = vec![vec![0isize; 3005]; 3005];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=n {
            if s[i] == '(' {
                dp[i + 1][j + 1] = dp[i][j];
                dp[i + 1][j + 1] %= MOD;
            } else if s[i] == ')' {
                if j > 0 {
                    dp[i + 1][j - 1] = dp[i][j];
                    dp[i + 1][j - 1] %= MOD;
                }
            } else {
                dp[i + 1][j + 1] = dp[i][j];
                dp[i + 1][j + 1] %= MOD;
                if j > 0 {
                    dp[i + 1][j - 1] += dp[i][j];
                    dp[i + 1][j - 1] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[n][0]);
}
