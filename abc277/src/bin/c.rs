use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_by_key(|v| v.0);

    let mut a = vec![];
    let mut b = vec![];

    for i in 0..n {
        a.push(ab[i].0);
        b.push(ab[i].1);
    }

    let mut visible = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(1);

    let mut map = HashMap::new();

    for i in 0..n {
        map.entry(a[i]).or_insert(vec![]).push(b[i]);
        map.entry(b[i]).or_insert(vec![]).push(a[i]);
    }

    while !queue.is_empty() {
        let v = queue.pop_back().unwrap();

        if let Some(_) = visible.get(&v) {
            continue;
        }

        if let Some(x) = map.get(&v) {
            for i in 0..x.len() {
                queue.push_back(x[i]);
            }
        }

        visible.insert(v, true);
    }

    println!("{}", visible.keys().max().unwrap());
}
