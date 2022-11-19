use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        _r: usize,
        _l: usize,
        x: [usize; n],
    };

    let _mod = 1_000_000_007;
    let mut dp = vec![0; n + 1];
    let mut sum = vec![0; n + 1];
    let mut _x = vec![0; n + 2];

    for i in 0..n {
        _x[i + 1] = x[i];
    }
    _x[n + 1] = w;

    dp[0] = 1;
    sum[0] = 1;
}
