use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize
        };

        if t == 1 {
            input! {
                x: String,
            };
            queue.push_back(x);
        } else if t == 2 {
            println!("{}", queue.front().unwrap());
        } else {
            queue.pop_front().unwrap();
        }
    }
}
