use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };

    let mut s = vec![];

    for (a, b) in ab {
        for _ in 0..b {
            s.push(a);
        }
    }

    let len = s.len();
    let mut dp = vec![vec![false; x + 1]; len + 1];
    dp[0][0] = true;

    for i in 1..=len {
        for j in 0..=x {
            let v = s[i - 1];
            let next = j + v;
            if next <= x && dp[i - 1][j] {
                dp[i][next] = true;
            }
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
        }
    }

    for vec in dp {
        if vec[x] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
