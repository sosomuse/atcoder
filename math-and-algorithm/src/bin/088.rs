use proconio::input;

const MOD: usize = 998244353;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    let ans = solve(a % MOD) * solve(b % MOD) % MOD * solve(c % MOD);

    println!("{}", ans % MOD);
}

fn solve(n: usize) -> usize {
    (n * (n + 1) / 2) % MOD
}
