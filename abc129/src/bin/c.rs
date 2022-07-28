use std::i64::MAX;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut dp: Vec<i64> = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for v in a.iter() {
        dp[*v] = -MAX;
    }

    for i in 2..=n {
        let v1 = dp[i - 1];
        let v2 = dp[i - 2];

        if v1 > 0 {
            dp[i] += v1;
        }

        if v2 > 0 {
            dp[i] += v2;
        }

        if dp[i] > 0 {
            dp[i] %= 1_000_000_007;
        }
    }

    println!("{}", dp[n]);
}
