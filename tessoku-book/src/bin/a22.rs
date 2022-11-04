use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    };

    let mut dp = vec![0; n + 1];
    let mut queue = VecDeque::new();
    queue.push_back(1);

    while let Some(i) = queue.pop_back() {
        let x = a[i - 1];
        let y = b[i - 1];

        dp[x] = dp[x].max(dp[i] + 100);
        dp[y] = dp[y].max(dp[i] + 150);

        if x != n {
            queue.push_back(x);
        }

        if y != n {
            queue.push_back(y);
        }
    }

    println!("{}", dp[n]);
}
