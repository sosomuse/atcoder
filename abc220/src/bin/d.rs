use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = vec![[0_usize; 10]; n + 1];
    dp[1][a[0]] = 1;

    for i in 1..n {
        for j in 0..10 {
            dp[i + 1][solve1(j, a[i])] += dp[i][j];
            dp[i + 1][solve2(j, a[i])] += dp[i][j];

            dp[i + 1][solve1(j, a[i])] %= 998244353;
            dp[i + 1][solve2(j, a[i])] %= 998244353;
        }
    }

    for i in 0..10 {
        println!("{}", dp[n][i]);
    }
}

fn solve1(a: usize, b: usize) -> usize {
    (a + b) % 10
}

fn solve2(a: usize, b: usize) -> usize {
    (a * b) % 10
}
