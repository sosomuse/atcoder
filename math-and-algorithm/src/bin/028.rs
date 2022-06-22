use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![0; n + 1];

    for i in 0..n {
        let v = h[i];
        if i == 0 {
            dp[i] = 0;
        }

        if i == 1 {
            dp[i] = (h[i - 1] - v).abs();
        }

        if i >= 2 {
            let dp1 = dp[i - 1] + (h[i - 1] - v).abs();
            let dp2 = dp[i - 2] + (h[i - 2] - v).abs();
            dp[i] = dp1.min(dp2);
        }
    }

    println!("{}", dp[n - 1]);
}
