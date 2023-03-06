use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;

    for i in 1..=n {
        let mut dp = vec![vec![vec![0usize; i]; i + 1]; n + 1];
        dp[0][0][0] = 1;

        for j in 0..n {
            for k in 0..=i {
                for l in 0..i {
                    dp[j + 1][k][l] += dp[j][k][l];
                    dp[j + 1][k][l] %= 998244353;
                    if k != i {
                        dp[j + 1][k + 1][(l + a[j]) % i] += dp[j][k][l];
                        dp[j + 1][k + 1][(l + a[j]) % i] %= 998244353;
                    }
                }
            }
        }
        ans += dp[n][i][0];
        ans %= 998244353;
    }

    println!("{}", ans);
}
