use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_a
fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    // dpの箱を作成（n+1）
    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        let v = h[i - 1];
        // 最初は0
        if i == 1 {
            dp[i] = 0;
        }

        // 次の値は一つ前と比較
        if i == 2 {
            dp[i] = (h[i - 2] - v).abs();
        }

        // 一つ前と2つ前の値を比較
        if i >= 3 {
            let dp1 = dp[i - 1] + (h[i - 2] - v).abs();
            let dp2 = dp[i - 2] + (h[i - 3] - v).abs();
            dp[i] = dp1.min(dp2);
        }
    }

    // 最後の要素が答え
    println!("{}", dp[n]);
}
