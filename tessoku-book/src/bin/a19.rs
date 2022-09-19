use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wu: [(usize, usize); n],
    };

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=w {
            let (w2, u2) = wu[i - 1];

            if j >= w2 {
                dp[i][j] = (dp[i - 1][j - w2] + u2).max(dp[i - 1][j]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    println!("{}", dp[n][w]);
}
