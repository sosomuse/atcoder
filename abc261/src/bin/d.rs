use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        cy: [(usize,usize); m],
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];
    let mut vec = vec![0; n + 1];

    for (c, y) in cy.iter() {
        vec[*c] = *y;
    }

    // 何回目のコイントスか
    for i in 1..=n {
        let v = x[i - 1];

        // 連続何回表が出たか
        for j in 1..=n {
            if i < j {
                break;
            }
            dp[i][0] = dp[i][0].max(dp[i - 1][j]);
            dp[i][j] = dp[i - 1][j - 1] + v + vec[j];
        }
    }

    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=n {
            ans = ans.max(dp[i][j]);
        }
    }

    println!("{}", ans);
}
