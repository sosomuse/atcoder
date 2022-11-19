use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        _: usize,
        q: usize,
    };

    let mut follow = HashMap::<usize, BTreeSet<usize>>::new();

    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        };

        match t {
            1 => {
                let set = follow.entry(a).or_insert(BTreeSet::new());
                set.insert(b);
            }
            2 => {
                let set = follow.entry(a).or_insert(BTreeSet::new());
                set.remove(&b);
            }
            3 => {
                let default = BTreeSet::<usize>::new();
                let a_set = follow.get(&a).unwrap_or(&default);
                let b_set = follow.get(&b).unwrap_or(&default);

                if a_set.contains(&b) && b_set.contains(&a) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}
