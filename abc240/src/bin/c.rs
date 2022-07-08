use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![vec![false; x]; n];
    dp[0][0] = true;

    for i in 0..n {
        let (a, b) = ab[i];
    }

    println!("{}", dp[0][0]);
}
