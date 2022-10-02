use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    };

    let max = 10001;

    let mut dp = vec![vec![false; max + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let (a, b) = ab[i - 1];
        for j in 0..=max {
            if dp[i - 1][j] {
                if j + a <= max {
                    dp[i][j + a] = true;
                }

                if j + b <= max {
                    dp[i][j + b] = true;
                }
            }
        }
    }

    if !dp[n][s] {
        println!("No");
        return;
    }

    let mut ans = vec![];
    let mut p = s;

    for i in (0..n).rev() {
        let (a, b) = ab[i];

        if p >= a && dp[i][p - a] {
            ans.push('H');
            p -= a;
        } else if p >= b && dp[i][p - b] {
            ans.push('T');
            p -= b;
        }
    }

    ans.reverse();

    println!("Yes");

    println!("{}", ans.iter().collect::<String>());
}
