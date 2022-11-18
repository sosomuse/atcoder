use proconio::{input, marker::Chars};

const A: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![0; 7]; n + 1];

    for i in 0..n {
        let v = s[i];
        for j in 0..7 {
            if let Some(p) = A.iter().position(|&x| x == v) {};

            dp[i + 1][j] = dp[i][j].max(dp[i + 1][j]);
        }
    }
}
