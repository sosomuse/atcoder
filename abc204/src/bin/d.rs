use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    };

    let sum = t.iter().sum::<usize>();

    let mut dp = vec![vec![false; sum + 1]; n + 1];
    dp[0][0] = true;

    for v in t {
        for i in 1..=n {
            for j in 1..=sum {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                }
                if j >= v && dp[i - 1][j - v] {
                    dp[i][j] = true;
                }
            }
        }
    }

    dbg!(dp);
}
