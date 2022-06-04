use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![0; n + 1];

    for i in 0..=n {
        if i <= 1 {
            dp[i] = 1;
        } else {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
    }

    dbg!(&dp);

    println!("{}", dp[n]);
}
