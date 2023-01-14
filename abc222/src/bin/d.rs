use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let mut c = vec![0; n];
    let mut s = vec![0; n];

    for i in 0..n {
        c[i] = b[i] - a[i];
    }

    for i in (0..n).rev() {
        if i == n - 1 {
            s[i] = c[i] + 1;
        } else {
            s[i] = b[i + 1] * c[i];
            s[i] %= 998244353;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans += s[i];
        ans %= 998244353;
    }

    dbg!(s);

    println!("{}", ans);
}
