use proconio::input;

fn ch_max(a: &mut isize, b: isize) {
    *a = std::cmp::max(*a, b);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![vec![-1_isize; d + 1]; k + 5]; n + 5];
    dp[0][0][0] = 0;

    for i in 0..n {
        let ni = i + 1;
        let v = a[i];
        for j in 0..=k {
            for r in 0..d {
                let now = dp[i][j][r];

                if now == -1 {
                    continue;
                }

                {
                    let nj = j + 1;
                    let nk = (r + v) % d;
                    let c = now + v as isize;
                    ch_max(&mut dp[ni][nj][nk], c)
                }

                {
                    let nj = j;
                    let nk = r;
                    let c = now;
                    ch_max(&mut dp[ni][nj][nk], c)
                }
            }
        }
    }

    println!("{}", dp[n][k][0]);
}
