use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        (a, b, c, d, e, f): (isize, isize, isize, isize, isize, isize),
        xy: [(isize, isize); m],
    };

    let mod_ = 998244353;
    let mut ans = 1;

    for _ in 0..n {
        ans += 3;
        ans %= mod_;
    }

    println!("{}", ans);
}
