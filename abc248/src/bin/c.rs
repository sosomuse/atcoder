use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    let _mod = 998244353;
    let mut dp = vec![vec![0; K + 1]; N + 1];
    dp[0][0] = 1;

    for i in 0..N {
        for j in 0..=K {
            for a in 1..=M {
                if j + a <= K {
                    dp[i + 1][(j + a)] += dp[i][j];
                    dp[i + 1][(j + a)] %= _mod;
                }
            }
        }
    }
    let ans = dp[N].iter().sum::<u64>() % _mod;
    println!("{}", ans);
}
