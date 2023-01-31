use proconio::input;

fn main() {
    input! {
        s: usize,
    };

    let mut dp: Vec<usize> = vec![0; s + 10];

    dp[0] = 1;
    dp[1] = 0;
    dp[2] = 0;

    for i in 3..=s {
        dp[i] = (dp[i - 1] + dp[i - 3]) % 1_000_000_007;
    }

    println!("{}", dp[s]);
}
