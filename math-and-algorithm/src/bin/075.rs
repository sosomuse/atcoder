use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;

    for i in 0..n {
        let t = {
            if i >= (n - 1) / 2 {
                n - i - 1
            } else {
                i
            }
        };

        ans += a[i] * ncr(n - 1, t, 1000000007);
        ans %= 1000000007;
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
