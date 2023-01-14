use std::collections::BTreeMap;

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        C: usize,
        mut abc: [(usize, usize, isize); n],
    };

    abc.sort_by_key(|&(a, b, _)| (a, b));

    let mut m = BTreeMap::new();

    for (a, b, c) in abc {
        *m.entry(a).or_insert(0) += c;
        *m.entry(b + 1).or_insert(0) -= c;
    }

    let mut t = 0;
    let mut prev = 0;
    let mut ans = 0;

    for (a, c) in m.iter() {
        if prev != 0 {
            ans += (a - prev) as isize * t.min(C as isize);
        }

        t += c;
        prev = *a;
    }

    println!("{}", ans);
}
