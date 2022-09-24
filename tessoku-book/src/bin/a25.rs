use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        cw: [Chars; h],
    };

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            let v = cw[i - 1][j - 1];
            if v == '#' {
                continue;
            }

            if i == 1 && j == 1 {
                dp[1][1] = 1;
                continue;
            }

            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    println!("{}", dp[h][w]);
}
