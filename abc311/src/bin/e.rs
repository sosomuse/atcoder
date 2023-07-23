use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1, Usize1); n]
    }

    let mut grid = vec![vec![1; w + 1]; h + 1];
    for (a, b) in ab {
        grid[a][b] = 0;
    }

    let mut dp = vec![vec![0; w + 1]; h + 1];
    let mut ans = 0usize;

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 1 {
                if i > 0 && j > 0 {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                } else {
                    dp[i][j] = 1;
                }
                ans += dp[i][j];
            }
        }
    }

    println!("{}", ans);
}
