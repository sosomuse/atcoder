use proconio::input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    let mut memo = HashMap::new();

    for i in 0..n {
        heap.push(Reverse(a[i]));
    }

    while k > 0 {
        let v = heap.pop().unwrap().0;

        if let Some(_) = memo.get(&v) {
            continue;
        }

        memo.insert(v, true);

        for i in 0..n {
            heap.push(Reverse(v + a[i]));
        }

        k -= 1;

        if k == 0 {
            println!("{}", v);
        }
    }
}
