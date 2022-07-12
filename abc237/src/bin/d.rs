use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut queue = VecDeque::<usize>::new();
    queue.push_back(n);

    for i in (0..n).rev() {
        let c = s[i];

        match c {
            'L' => {
                queue.push_back(i);
            }
            'R' => {
                queue.push_front(i);
            }
            _ => unimplemented!(),
        }
    }

    for i in 0..=n {
        print!("{} ", queue[i]);
    }
}
