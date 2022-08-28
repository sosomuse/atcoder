use proconio::input;

fn main() {
    input! {
        n: usize,
        mut txa: [(usize, usize, usize); n],
    };

    txa.sort_by_key(|(t, _, _)| *t);

    let mut max_t = 0;

    for (ti, _, _) in txa.iter() {
        max_t = max_t.max(*ti);
    }

    // i = 時間, j = 座標
    let mut dp = vec![vec![0; 5]; max_t + 1];
    let mut count = 0;

    for i in 1..=max_t {
        let (t, x, a) = txa[count];
        if t == i {
            count += 1;

            for j in 0..=4 {
                let mut max = 0;

                for k in 0.max(j as isize - 1) as usize..=(j + 1).min(4) {
                    max = max.max(dp[i - 1][k]);
                }

                dp[i][j] = max;
                if x == j && t >= j {
                    dp[i][j] += a;
                }
            }
        } else {
            for j in 0..=4 {
                let mut max = 0;

                for k in 0.max(j as isize - 1) as usize..=(j + 1).min(4) {
                    max = max.max(dp[i - 1][k]);
                }

                dp[i][j] = max;
            }
        }
    }

    let mut ans = 0;

    for i in 0..=4 {
        ans = ans.max(dp[max_t][i]);
    }

    println!("{}", ans);
}
