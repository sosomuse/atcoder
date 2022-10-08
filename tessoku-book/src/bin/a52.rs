use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut deque = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize
        };

        if t == 1 {
            input! {
                x: String,
            };
            deque.push_back(x);
        } else if t == 2 {
            println!("{}", deque.front().unwrap());
        } else {
            deque.pop_front().unwrap();
        }
    }
}
