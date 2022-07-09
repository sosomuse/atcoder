use std::collections::BTreeSet;

use proconio::input;

// #![feature(repr128)]

// #[repr(u128)] // ok!
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    let mut btree = BTreeSet::new();

    let v = p[0..k].to_vec();

    for i in v {
        btree.insert(i);
    }

    println!("{}", *btree.iter().next().unwrap());

    for i in 0..n - k {
        let v = p[i + k];
        let next = *btree.iter().next().unwrap();

        if v > next {
            btree.insert(v);
            btree.remove(&next);
        }

        println!("{}", *btree.iter().next().unwrap());
    }
}
