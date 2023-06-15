use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        x: usize,
    };

    let mut ans = 1;
    let mut tmp = 1;

    for _ in 0..x {
        ans *= 10;
        ans %= MOD;
    }
    for _ in 0..x {
        tmp *= 9;
        tmp %= MOD;
    }
    tmp *= 2;
    tmp %= MOD;

    ans = (ans + MOD - tmp) % MOD;

    tmp = 1;

    for _ in 0..x {
        tmp *= 8;
        tmp %= MOD;
    }

    ans += tmp;
    ans %= MOD;

    println!("{}", ans);
}
