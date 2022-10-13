use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![0; n + 1]; 32];

    for i in 0..n {
        dp[0][i + 1] = a[i];
    }

    for d in 1..30 {
        for i in 1..=n {
            dp[d][i] = dp[d - 1][dp[d - 1][i]];
        }
    }

    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        };

        let mut cp = x;

        for d in (0..30).rev() {
            if (y / (1 << d)) % 2 != 0 {
                cp = dp[d][cp];
            }
        }

        println!("{}", cp);
    }
}
