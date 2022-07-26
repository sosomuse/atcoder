use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut map = HashMap::<String, bool>::new();

    for v in t.iter() {
        map.insert(v.to_string(), true);
    }

    for v in s.iter() {
        if map.contains_key(v) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
