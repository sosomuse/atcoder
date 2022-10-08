use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut pairs: Vec<(usize, usize)> = vec![];
    let mut queue: VecDeque<usize> = VecDeque::new();

    for i in 0..s.len() {
        if s[i] == '(' {
            queue.push_back(i + 1)
        } else {
            pairs.push((queue.pop_back().unwrap(), i + 1));
        }
    }

    for i in 0..pairs.len() {
        println!("{} {}", pairs[i].0, pairs[i].1);
    }
}
