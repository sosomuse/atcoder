use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut dp = vec![0usize; 8];

    for i in 0..s.len() {
        match s[i] {
            'c' => {
                dp[0] += 1;
            }
            'h' => {
                dp[1] += dp[0];
            }
            'o' => {
                dp[2] += dp[1];
            }
            'k' => {
                dp[3] += dp[2];
            }
            'u' => {
                dp[4] += dp[3];
            }
            'd' => {
                dp[5] += dp[4];
            }
            'a' => {
                dp[6] += dp[5];
            }
            'i' => {
                dp[7] += dp[6];
            }
            _ => continue,
        }

        for j in 0..8 {
            dp[j] %= 1_000_000_007;
        }
    }

    println!("{}", dp[7]);
}
