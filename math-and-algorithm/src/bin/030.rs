use proconio::*;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    // dbg!(&dp);
    // dbg!(&wv);

    for i in 0..n {
        for j in 0..=w {
            if j >= wv[i].0 {
                dp[i + 1][j] = (dp[i][j - wv[i].0] + wv[i].1).max(dp[i][j]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println!("{}", dp[n][w - 1]);
}
