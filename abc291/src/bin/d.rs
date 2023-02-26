use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![vec![0usize; 2]; n + 1];
    dp[1][0] = 1;
    dp[1][1] = 1;

    for i in 1..n {
        let (a, b) = ab[i];
        let (c, d) = ab[i - 1];

        if a != c {
            dp[i + 1][0] += dp[i][0];
        }

        if a != d {
            dp[i + 1][0] += dp[i][1];
        }

        if b != c {
            dp[i + 1][1] += dp[i][0];
        }

        if b != d {
            dp[i + 1][1] += dp[i][1];
        }

        dp[i + 1][0] %= 998244353;
        dp[i + 1][1] %= 998244353;
    }

    println!("{}", (dp[n][0] + dp[n][1]) % 998244353);
}
