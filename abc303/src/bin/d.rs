use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
        s: Chars,
    }

    let mut dp = vec![vec![0; 2]; s.len() + 1];
    dp[0][1] = z;

    for i in 0..s.len() {
        let ch = s[i];

        dp[i + 1][0] = std::cmp::min(
            dp[i][0] + (if ch == 'a' { x } else { y }),
            dp[i][1] + z + (if ch == 'a' { x } else { y }),
        );

        dp[i + 1][1] = std::cmp::min(
            dp[i][1] + (if ch == 'a' { y } else { x }),
            dp[i][0] + z + (if ch == 'a' { y } else { x }),
        );
    }

    println!("{}", std::cmp::min(dp[s.len()][0], dp[s.len()][1]));
}
