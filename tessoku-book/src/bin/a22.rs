use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    };

    let mut dp = vec![-100000; n + 1];
    dp[1] = 0;

    for i in 1..n {
        let x = a[i - 1];
        let y = b[i - 1];

        dp[x] = dp[x].max(dp[i] + 100);
        dp[y] = dp[y].max(dp[i] + 150);
    }

    println!("{}", dp[n]);
}
