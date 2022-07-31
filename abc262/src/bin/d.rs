use proconio::input;

fn main() {
    input! {
        n: usize,
        // a: [usize; n],
    };

    let mut ans = 0;

    for i in 1..=n {
        ans += ncr(n, i, 998244353);
        ans %= 998244353;
    }

    println!("{}", ans);
}

fn ncr(n: usize, r: usize, m: usize) -> usize {
    let mut _n = 1;
    let mut _r = 1;

    for i in 0..r {
        _n *= n - i;
        _n %= m;
        _r *= i + 1;
        _r %= m;
    }

    _n / _r
}
