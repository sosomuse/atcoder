use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let (a, b) = ab[i - 1];

        for j in 0..=x {
            if j >= a && dp[i - 1][j - a] {
                dp[i][j] = true;
            }

            if j >= b && dp[i - 1][j - b] {
                dp[i][j] = true;
            }
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
