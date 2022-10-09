use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut binary_heap = BinaryHeap::new();

    for _ in 0..q {
        input! {
            t: usize,
        };

        match t {
            1 => {
                input! {
                    x: usize,
                };

                binary_heap.push(Reverse(x));
            }
            2 => {
                println!("{}", binary_heap.peek().unwrap().0);
            }
            3 => {
                binary_heap.pop().unwrap();
            }
            _ => unreachable!(),
        }
    }
}
