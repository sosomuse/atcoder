use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    };

    let mut ans: usize = 0;

    for i in 0..n {
        ans += a[i] * m;
    }

    ans += b * n * m;

    for i in 0..m {
        ans += c[i] * n;
    }

    println!("{}", ans);
}
