use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    };

    let mut dp = vec![vec![0; 3]; n + 1];
    let mut dp2 = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            dp[i + 1][j] = dp[i][j];
        }
        if s[i] == 'M' {
            dp[i + 1][a[i]] += 1;
        }
    }

    for i in (0..n).rev() {
        for j in 0..3 {
            dp2[i][j] = dp2[i + 1][j];
        }
        if s[i] == 'X' {
            dp2[i][a[i]] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if s[i] != 'E' {
            continue;
        }
        for j in 0..3 {
            for k in 0..3 {
                ans += dp[i][j] * dp2[i + 1][k] * mex(j, a[i], k);
            }
        }
    }

    println!("{}", ans);
}

fn mex(x: usize, y: usize, z: usize) -> usize {
    for i in 0..3 {
        if x != i && y != i && z != i {
            return i;
        }
    }
    3
}
