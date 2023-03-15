use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        x: usize,
        y: usize,
    };

    if (x + y) % 3 != 0 {
        println!("0");
        return;
    }

    if x * 2 < y || y * 2 < x {
        println!("0");
        return;
    }

    let n = (x * 2 - y) / 3;
    let m = (y * 2 - x) / 3;

    let mut ans = 1;
    for i in 0..n {
        ans *= n + m - i;
        ans %= MOD;
    }
    let mut d = 1;
    for i in 0..n {
        d *= i + 1;
        d %= MOD;
    }
    ans *= pow(d, MOD - 2);
    ans %= MOD;
    println!("{}", ans);
}

fn pow(x: usize, y: usize) -> usize {
    let mut p = x;
    let mut ans = 1;
    for i in 0..=30 {
        if (y >> i) & 1 == 1 {
            ans *= p;
            ans %= MOD;
        }
        p *= p;
        p %= MOD;
    }
    ans
}
