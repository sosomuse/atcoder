use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        a: [usize; n],
    };

    let mut heap = BinaryHeap::new();

    for v in a {
        heap.push(v);
    }

    for _ in 0..m {
        let v = heap.pop().unwrap();
        heap.push(v / 2);
    }

    let mut sum = 0;

    for v in heap {
        sum += v;
    }

    println!("{}", sum);
}
