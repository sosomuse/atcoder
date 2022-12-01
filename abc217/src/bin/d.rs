use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
    };

    let mut s = BTreeSet::new();
    s.insert(0);
    s.insert(l);

    for _ in 0..q {
        input! {
            c: usize,
            x: usize,
        }

        if c == 1 {
            s.insert(x);
        } else {
            let r = s.range(x..).next().unwrap();
            let l = s.range(..x).rev().next().unwrap();
            println!("{}", r - l);
        }
    }
}
