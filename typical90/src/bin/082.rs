use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let mut ans = 0;
    let min_len = l.to_string().len();
    let max_len = r.to_string().len();

    for i in min_len..=max_len {
        let min = 10usize.pow(i as u32 - 1);
        let max = 10usize.pow(i as u32) - 1;
        let min = std::cmp::max(min, l);
        let max = std::cmp::min(max, r);

        if min > max {
            continue;
        }

        let mut tmp = mod_pow(max, max / 2, MOD) * i % MOD;
        tmp += MOD;
        tmp -= mod_pow(min - 1, min / 2, MOD) * i % MOD;
        tmp %= MOD;

        ans = (ans + tmp) % MOD;
    }

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
