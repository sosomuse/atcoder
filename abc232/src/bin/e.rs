use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    };

    let mut dp = vec![vec![0; 4]; k + 1];
    if x1 == x2 && y1 == y2 {
        dp[0][0] = 1;
    } else if x1 == x2 {
        dp[0][1] = 1;
    } else if y1 == y2 {
        dp[0][2] = 1;
    } else {
        dp[0][3] = 1;
    }

    for i in 0..k {
        dp[i + 1][0] += dp[i][1];
        dp[i + 1][0] += dp[i][2];

        dp[i + 1][1] += dp[i][0] * (w - 1);
        dp[i + 1][1] += dp[i][1] * (w - 2);
        dp[i + 1][1] += dp[i][3];

        dp[i + 1][2] += dp[i][0] * (h - 1);
        dp[i + 1][2] += dp[i][2] * (h - 2);
        dp[i + 1][2] += dp[i][3];

        dp[i + 1][3] += dp[i][3] * (h + w - 4);
        dp[i + 1][3] += dp[i][1] * (h - 1);
        dp[i + 1][3] += dp[i][2] * (w - 1);

        for j in 0..4 {
            dp[i + 1][j] %= MOD;
        }
    }

    println!("{}", dp[k][0]);
}
