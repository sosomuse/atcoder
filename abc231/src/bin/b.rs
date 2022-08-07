use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut map = HashMap::<String, usize>::new();

    for v in s.iter() {
        *map.entry(v.to_string()).or_insert(0) += 1;
    }

    let max = map.values().max().unwrap();

    for (k, v) in map.iter() {
        if *v == *max {
            println!("{}", k);
            return;
        }
    }
}
