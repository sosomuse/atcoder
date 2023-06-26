use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut dp = vec![vec![0; 2]; n + 1];
    dp[1][1] = m;

    for i in 1..n {
        dp[i + 1][0] += dp[i][0] * (m - 2);
        dp[i + 1][1] += dp[i][0];
        dp[i + 1][0] += dp[i][1] * (m - 1);
        dp[i + 1][0] %= MOD;
        dp[i + 1][1] %= MOD;
    }

    println!("{}", dp[n][0]);
}
