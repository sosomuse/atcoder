use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    };

    let mut dp = vec![100000000000; n + 1];
    dp[1] = 0;

    for i in 2..=n {
        if i == 2 {
            dp[i] = (h[i - 1] - h[i - 2]).abs();
        }
        for j in ((1).max((i as isize) - k as isize) as usize)..i {
            dp[i] = dp[i].min(dp[j] + (h[j - 1] - h[i - 1]).abs());
        }
    }

    println!("{}", dp[n]);
}
