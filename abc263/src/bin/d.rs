use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n],
    }

    let mut ans = r * n as isize;
    let mut suma = 0;
    let mut sumb = 0;
    let mut maxb = 0;

    for i in 0..n {
        suma += a[i];
        sumb += a[i] - l;
        maxb = max(maxb, sumb);
        let now = suma + r * (n as isize - i as isize - 1) - maxb;
        ans = min(ans, now);
    }

    println!("{}", ans);
}
