use proconio::input;

fn main() {
    input! {
        n: usize,
        dishes: [(usize, isize); n],
    }

    let mut dp = vec![vec![0; 2]; n + 1];

    for i in 0..n {
        let (x, y) = dishes[i];
        dp[i + 1][0] = dp[i][0];
        dp[i + 1][1] = dp[i][1];

        if x == 0 {
            dp[i + 1][0] = std::cmp::max(dp[i + 1][0], dp[i][0] + y);
            dp[i + 1][0] = std::cmp::max(dp[i + 1][0], dp[i][1] + y);
            dp[i + 1][1] = std::cmp::max(dp[i + 1][1], dp[i][1]);
        } else {
            dp[i + 1][1] = std::cmp::max(dp[i + 1][1], dp[i][0] + y);
            dp[i + 1][0] = std::cmp::max(dp[i + 1][0], dp[i][0]);
        }
    }

    println!("{}", std::cmp::max(dp[n][0], dp[n][1]));
}
