use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    }

    let mut dp = vec![0; n];
}

// 確率MOD
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
