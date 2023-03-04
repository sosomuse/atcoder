use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    };

    // dp[l][r] := l番目からr番目までの区間が残っている場合の得点
    let mut dp = vec![vec![0usize; n + 1]; n + 1];

    for l in 1..=n {
        for s in 1..l {
            let (p, a) = pa[s - 1];

            if s < p && l < p {
                dp[l][n] += a;
            }
        }

        for r in (l..n).rev() {
            let (p, a) = pa[r];
            dp[l][r] = dp[l][r + 1];

            if l < p && r < p {
                dp[l][r] += a;
            }
        }
    }

    dbg!(dp);
}
