use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    };

    let sum = a.iter().sum::<usize>();

    a.sort_by(|a, b| b.cmp(a));
    let mut count = HashMap::new();

    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }

    let unique: Vec<_> = a.into_iter().unique().collect();
    let mut map = HashMap::new();
    let mut dequeue = VecDeque::new();

    for i in 0..unique.len() {
        let v = unique[i];

        if let Some(x) = count.get(&v) {
            let mut y = v * x;

            if let Some(z) = map.get(&((v + 1) % m)) {
                y += z;
            } else {
                dequeue.push_back(v);
            }

            map.insert(v, y);
        }
    }

    while !dequeue.is_empty() {
        let v = dequeue.pop_back().unwrap();
        let mut z = 0;

        if let Some(x) = map.get(&((v + 1) % m)) {
            if *x < v {
                z = *x;
            }
        }

        if let Some(x) = map.get_mut(&v) {
            *x += z;
        }
    }

    let max = map.values().max().unwrap();

    println!("{}", sum - *max);
}
