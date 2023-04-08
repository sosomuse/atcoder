use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    if n == 1 {
        println!("{}", k);
        return;
    }

    let mut ans = mod_pow(k - 2, n - 2, MOD);
    ans = ans * (k - 1) % MOD;
    ans = ans * k % MOD;

    println!("{}", ans);
}

fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let mut r = 1;
    let mut p = a;
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            r = r * p % m;
        }
        p = p * p % m;
    }
    r
}
