use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _:usize,
        m:usize,
        py: [(usize, usize); m],
    };

    let mut map = HashMap::<usize, Vec<usize>>::new();

    for &(p, y) in &py {
        let v = map.entry(p).or_insert(Vec::new());
        v.push(y);
    }

    for v in map.values_mut() {
        v.sort();
    }

    for &(p, y) in &py {
        let v = map.get(&p).unwrap();
        let ans = v.binary_search(&y).unwrap_or_else(|x| x);

        println!("{:06}{:06}", p, ans + 1);
    }
}
