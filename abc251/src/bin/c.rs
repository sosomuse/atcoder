use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u32,
        a: [(String, u32); n],
    }

    let mut map: HashMap<String, u32> = HashMap::new();
    let c = a.clone();

    for (s, v) in a {
        map.entry(s).or_insert(v);
    }

    let max = map.values().max().unwrap();

    for (i, (s, v)) in c.into_iter().enumerate() {
        if v == *max && v == *map.get(&s).unwrap() {
            println!("{}", i + 1);
            break;
        }
    }
}
