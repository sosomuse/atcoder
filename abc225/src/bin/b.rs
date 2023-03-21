use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    };

    let mut trees = HashMap::new();
    for (a, b) in ab {
        let tree = trees.entry(a).or_insert(HashSet::new());
        tree.insert(b);

        let tree = trees.entry(b).or_insert(HashSet::new());
        tree.insert(a);
    }

    let max_len = trees.values().map(|tree| tree.len()).max().unwrap();

    if max_len == n - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
