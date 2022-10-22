use std::collections::BTreeSet;
// use std::ops::Bound::Included;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();

    for i in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        if t == 1 {
            set.insert((x, i));
        } else if t == 2 {
            input! {
                k: usize,
            }

            let mut elem = set.range(..=(x, q));

            if let Some(v) = elem.nth_back(k - 1) {
                println!("{}", v.0);
            } else {
                println!("-1");
            }
        } else {
            input! {
                k: usize,
            }

            let mut elem = set.range((x, 0)..);

            if let Some(v) = elem.nth(k - 1) {
                println!("{}", v.0);
            } else {
                println!("-1");
            }
        }
    }
}
