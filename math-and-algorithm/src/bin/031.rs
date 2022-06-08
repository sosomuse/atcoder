use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 0..n {
            let v = a[j];

            dp[i][j] = dp[i - 1][j] + v;
        }
    }

    dbg!(&dp);

    println!("1")
}
