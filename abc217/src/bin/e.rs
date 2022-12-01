use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut deque = VecDeque::new();
    let mut binary_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize,
            };

            deque.push_back(x);
        } else if t == 2 {
            if binary_heap.is_empty() {
                println!("{}", deque.pop_front().unwrap());
            } else {
                println!("{}", binary_heap.pop().unwrap().0);
            }
        } else {
            for _ in 0..deque.len() {
                let x = deque.pop_front().unwrap();
                binary_heap.push(Reverse(x));
            }
        }
    }
}
