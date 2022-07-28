use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
    };

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1];

        if i >= l {
            dp[i] += dp[i - l];
            dp[i] %= 10_u32.pow(9) + 7;
        }
    }

    println!("{}", dp[n]);
}
