use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = vec![0; n];
    let mut c = a[0];
    dp[0] = 1;

    for i in 1..n {
        let v = a[i];
        if v > c {
            dp[i] = dp[i - 1] + 1;
        } else {
            dp[i] = 1;
        }

        c = v;
    }

    println!("{}", dp.iter().max().unwrap());
}
