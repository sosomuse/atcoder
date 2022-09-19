use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }

            if dp[i - 1][j] && s >= j + a[i - 1] {
                dp[i][j + a[i - 1]] = true;
            }
        }
    }

    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
