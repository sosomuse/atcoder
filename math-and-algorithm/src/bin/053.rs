use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let _mod = 1000000007;

    println!("{}", modpow(4, n, _mod))
}

fn modpow(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        if b & (1 << i) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }

    ans
}
