use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![vec![100000000000; h + 1]; n + 1];
    dp[0][h] = 0;

    for i in 0..n {
        for j in (0..=h).rev() {
            let (a, b) = ab[i];
            dp[i + 1][j] = std::cmp::min(dp[i + 1][j], dp[i][j]);
            if j >= a {
                dp[i][j - a] = std::cmp::min(dp[i][j - a], dp[i][j] + b);
            } else {
                dp[i][0] = std::cmp::min(dp[i][0], dp[i][j] + b);
            }
        }
    }

    println!("{}", dp[n][0]);
}
