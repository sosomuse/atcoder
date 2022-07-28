use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [[usize; 3]; n],
    }

    let mut dp = vec![vec![0; 3]; n + 1];
    dp[1][0] = abc[0][0];
    dp[1][1] = abc[0][1];
    dp[1][2] = abc[0][2];

    for i in 2..=n {
        for j in 0..3 {
            let v = abc[i - 1][j];
            dp[i][j] = v + dp[i - 1][(j + 1) % 3].max(dp[i - 1][(j + 2) % 3]);
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
