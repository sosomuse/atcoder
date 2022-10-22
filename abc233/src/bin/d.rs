use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    };

    let mut s = vec![0; n + 1];
    let mut m = HashMap::new();

    for i in 0..n {
        s[i + 1] = a[i] + s[i];
    }

    let mut ans: isize = 0;

    for i in 1..=n {
        *m.entry([s[i - 1]]).or_insert(0) += 1;
        if let Some(v) = m.get(&[s[i] - k]) {
            ans += v;
        }
    }

    println!("{}", ans);
}
