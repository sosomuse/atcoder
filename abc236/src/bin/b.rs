use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 4 - 1],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for x in a {
        *map.entry(x).or_insert(0) += 1;
    }

    for (k, v) in map {
        if v % 4 != 0 {
            println!("{}", k)
        }
    }
}
