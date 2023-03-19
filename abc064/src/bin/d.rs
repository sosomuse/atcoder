use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    };

    let mut queue = VecDeque::new();
    let mut count = 0usize;

    for c in s {
        queue.push_back(c);

        if c == '(' {
            count += 1;
        } else if c == ')' {
            if count == 0 {
                queue.push_front('(');
            } else {
                count -= 1;
            }
        }
    }

    for _ in 0..count {
        queue.push_back(')');
    }

    println!("{}", queue.iter().collect::<String>());
}
