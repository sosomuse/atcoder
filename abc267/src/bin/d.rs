use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    };

    let mut dp: Vec<Vec<isize>> = vec![vec![0; n + 1]; n + 1];

    dp[0][1] = -1000000000000;

    for i in 1..=n {
        for j in 1..=n {
            if j == 0 {
                dp[i][0] = 0;
            } else if j > i {
                dp[i][j] = -1000000000000;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 1][j - 1] + a[i - 1] * j as isize);
            }
        }
    }

    println!("{}", dp[n][m]);
}
