use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        let v = a[i - 1];

        if i == 1 {
            dp[i] = v;
        }

        if i >= 2 {
            dp[i] = (dp[i - 1]).max(dp[i - 2] + v);
        }
    }

    println!("{}", dp[n]);
}
