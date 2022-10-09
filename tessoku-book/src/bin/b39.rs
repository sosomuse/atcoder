use proconio::input;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        d: usize,
        xy: [(usize, usize); n],
    };

    let mut job_map = HashMap::new();

    for (x, y) in xy {
        job_map.entry(x).or_insert(vec![]).push(y);
    }

    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    for i in 1..=d {
        if let Some(v) = job_map.get(&i) {
            for y in v {
                heap.push(*y);
            }
        }

        if let Some(z) = heap.pop() {
            ans += z;
        }
    }

    println!("{}", ans);
}
