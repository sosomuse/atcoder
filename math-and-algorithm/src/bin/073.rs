use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    let mut _mod = 1000000007;

    for i in 0..n {
        let v = a[i];
        ans += (v * mod_pow(2, i, _mod)) % _mod;
    }

    println!("{}", ans % _mod);
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
