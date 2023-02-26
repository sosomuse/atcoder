use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut ans = 0;

    for i in k..=n + 1 {
        let min = i * (i - 1) / 2;
        let max = i * (2 * n - i + 1) / 2;
        ans += max - min + 1;
        ans %= 1_000_000_007;
    }

    println!("{}", ans);
}
