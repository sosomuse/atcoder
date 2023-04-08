use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    if k % 9 != 0 {
        println!("0");
        return;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=k {
        let b = std::cmp::min(i, 9);
        for j in 1..=b {
            if i < j {
                continue;
            }

            dp[i] += dp[i - j];
            dp[i] %= 1_000_000_007;
        }
    }

    println!("{}", dp[k]);
}
