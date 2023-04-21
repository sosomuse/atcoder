use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        p: usize,
    };

    let mut dp = vec![0; n + 1];
    let critical = probability(p, 100);
    let normal = probability(100 - p, 100);

    dp[0] = 1;
    for i in 1..=n {
        dp[i] = (dp[0.max(i - 2)] * critical + dp[i - 1] * normal) + 1;
    }

    println!("{}", dp[n]);
}

fn probability(x: usize, y: usize) -> usize {
    let inv_y = mod_inv(y, MOD);
    (x * inv_y) % MOD
}

fn mod_inv(a: usize, m: usize) -> usize {
    mod_pow(a, m - 2, m)
}

fn mod_pow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        n /= 2;
    }
    res
}
