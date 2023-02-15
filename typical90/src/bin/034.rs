use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut set = HashSet::new();
    let mut counts = HashMap::new();

    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;

    for _ in 0..n {
        while r < n && set.len() <= k {
            if set.len() == k && !set.contains(&a[r]) {
                break;
            }

            *counts.entry(a[r]).or_insert(0) += 1;
            set.insert(a[r]);
            r += 1;
        }

        if set.len() <= k {
            ans = ans.max(r - l);
        }

        let current = set.len();

        while l < n && set.len() == current {
            *counts.entry(a[l]).or_insert(0) -= 1;
            if counts[&a[l]] == 0 {
                set.remove(&a[l]);
            }
            l += 1;
        }
    }

    println!("{}", ans);
}
