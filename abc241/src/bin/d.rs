use std::collections::BTreeSet;
// use std::ops::Bound::Included;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut set: BTreeSet<(isize, usize)> = BTreeSet::new();

    for i in 0..q {
        input! {
            t: usize,
            x: isize,
        };

        if t == 1 {
            set.insert((x, i));
        } else if t == 2 {
            input! {
                k: isize,
            }

            let mut elem = set.range((0, 0)..=(x, t)).rev();
            for j in 1..=k {
                if j == k {
                    if let Some(v) = elem.next() {
                        println!("{}", v.0);
                    } else {
                        println!("-1");
                    }
                } else {
                    elem.next();
                }
            }
        } else {
            input! {
                k: usize,
            }

            let mut elem = set.range((x, 0)..);
            // dbg!(&elem);
            for j in 1..=k {
                if j == k {
                    if let Some(v) = elem.next() {
                        println!("{}", v.0);
                    } else {
                        println!("-1");
                    }
                } else {
                    elem.next();
                }
            }
        }
    }
}
