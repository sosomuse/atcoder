use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        q: usize,
    };

    let mut s = vec![1];
    let mut r = 0;
    let mut base = 1;
    let mut len = 1;
    let mut total = 1;

    for _ in 0..q {
        input! {
            t: usize
        };

        match t {
            1 => {
                input! {
                    x: usize,
                };

                len += 1;
                s.push(x);
                base = mod_pow(10, len - 1, MOD) * s[r] % MOD;
                total = (total * 10 + x) % MOD;
            }
            2 => {
                total = (total + MOD - base) % MOD;
                r += 1;
                len -= 1;
                base = mod_pow(10, len - 1, MOD) * s[r] % MOD;
            }
            3 => {
                println!("{}", total)
            }
            _ => {}
        }
    }
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
