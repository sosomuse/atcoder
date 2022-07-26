use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut set = HashSet::<String>::new();

    for v in t.iter() {
        set.insert(v.to_string());
    }

    for v in s.iter() {
        if set.contains(v) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
