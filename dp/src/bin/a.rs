use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    };

    let mut dp = vec![0; n + 1];
    dp[1] = 0;

    for i in 2..=n {
        if i == 2 {
            dp[i] = dp[i - 1] + (h[i - 2] - h[i - 1]).abs();
        } else {
            dp[i] = (dp[i - 1] + (h[i - 2] - h[i - 1]).abs())
                .min(dp[i - 2] + (h[i - 3] - h[i - 1]).abs());
        }
    }

    println!("{}", dp[n]);
}
