use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wu: [(usize, usize); n],
    };

    let max = 1000 * 100;

    let mut dp = vec![vec![w + 1; max + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=max {
            let (w2, u2) = wu[i - 1];

            dp[i][j] = dp[i - 1][j].min(dp[i][j]);

            if u2 + j <= max {
                dp[i][u2 + j] = dp[i][u2 + j].min(dp[i - 1][j] + w2);
            }
        }
    }

    let mut ans = 0;

    for i in (0..=max).rev() {
        if dp[n][i] != w + 1 {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
