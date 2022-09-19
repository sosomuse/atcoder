use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }

            if dp[i - 1][j] && s >= j + a[i - 1] {
                dp[i][j + a[i - 1]] = true;
            }
        }
    }

    let exist = dp[n][s];

    if !exist {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    let mut p = s;

    for i in (0..n).rev() {
        if p >= a[i] && dp[i][p - a[i]] {
            ans.push(i + 1);
            p -= a[i];
        }
    }

    ans.reverse();

    println!("{}", ans.len());
    for i in ans {
        print!("{} ", i);
    }
}
