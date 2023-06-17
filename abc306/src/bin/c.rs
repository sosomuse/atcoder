use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; 3 * n]
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        map.entry(x).or_insert_with(Vec::new).push(i);
    }

    let mut pairs: Vec<(usize, usize)> = map
        .into_iter()
        .map(|(x, mut v)| {
            v.sort();
            (v[1], x)
        })
        .collect();

    pairs.sort();

    for pair in pairs {
        println!("{}", pair.1);
    }
}
