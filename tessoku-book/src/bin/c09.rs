use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = vec![0; n + 1];
    dp[1] = a[0];

    for i in 2..=n {
        let v = a[i - 1];
        dp[i] = dp[i - 1].max(dp[i - 2] + v);
    }

    println!("{}", dp[n]);
}
