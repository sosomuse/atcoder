use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let max = 3000;
    let mut dp = vec![vec![0_usize; max + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..=n {
        for j in 0..max {
            dp[i][j + 1] += dp[i][j];
            dp[i][j + 1] %= 998244353;
        }

        if i != n {
            for j in a[i]..=b[i] {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= 998244353;
            }
        }
    }

    println!("{}", dp[n][max]);
}
