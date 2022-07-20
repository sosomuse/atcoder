use proconio::input;

static MOD: u32 = 998244353;
fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![0; 10]; n + 1];

    for i in 1..=9 {
        dp[1][i] = 1;
    }

    for d in 2..=n {
        for i in 1..=9 {
            for j in 1.max(i - 1)..=9.min(i + 1) {
                dp[d][j] += dp[d - 1][i];
                dp[d][j] %= MOD;
            }
        }
    }

    let mut ans = 0;

    for i in 1..=9 {
        ans += dp[n][i];
        ans %= MOD;
    }

    println!("{}", ans);
}
