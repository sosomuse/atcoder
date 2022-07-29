use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wu: [(usize, usize); n],
    };

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        let (u, v) = wu[i - 1];
        dp[1][u] = dp[1][u].max(v);
    }

    dbg!(&dp);

    // for i in 2..=n {
    //     for j in 2..=w {
    //         let v = wu[i - 1].0;
    //         if j >= v {
    //             dp[i][j] = dp[i - 1][j - v] + wu[i - 1].1;
    //         } else {
    //             dp[i][j] = dp[i - 1][j];
    //         }
    //     }
    // }
}
