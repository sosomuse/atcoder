use proconio::input;

const MAX: usize = std::usize::MAX;

fn main() {
    input! {
        n: usize,
    };

    let mut draws = vec![];
    draws.push(1);

    let mut count = 1;

    while 6usize.pow(count) <= n {
        draws.push(6usize.pow(count));
        count += 1;
    }

    let mut count = 1;

    while 9usize.pow(count) <= n {
        draws.push(9usize.pow(count));
        count += 1;
    }

    draws.sort();

    let mut dp = vec![MAX; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        for &draw in &draws {
            if i >= draw {
                dp[i] = std::cmp::min(dp[i], dp[i - draw] + 1);
            }
        }
    }

    println!("{}", dp[n]);
}
