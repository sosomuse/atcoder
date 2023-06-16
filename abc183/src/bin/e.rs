use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut dp = vec![vec![0; w]; h];
    let mut x = vec![vec![0; w]; h];
    let mut y = vec![vec![0; w]; h];
    let mut z = vec![vec![0; w]; h];
    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }

            if s[i][j] == '#' {
                continue;
            }

            if j > 0 {
                x[i][j] = (x[i][j - 1] + dp[i][j - 1]) % MOD;
            }

            if i > 0 {
                y[i][j] = (y[i - 1][j] + dp[i - 1][j]) % MOD;
            }

            if i > 0 && j > 0 {
                z[i][j] = (z[i - 1][j - 1] + dp[i - 1][j - 1]) % MOD;
            }

            dp[i][j] = ((x[i][j] + y[i][j]) % MOD + z[i][j]) % MOD;
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
