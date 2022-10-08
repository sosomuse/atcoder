use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: Chars,
    };

    // #: Black, .: White

    let mut deque: VecDeque<usize> = VecDeque::with_capacity(n);

    deque.push_front(x - 1);
    a[x - 1] = '@';

    while !deque.is_empty() {
        if let Some(pos) = deque.pop_front() {
            if pos > 0 && a[pos - 1] == '.' {
                a[pos - 1] = '@';
                deque.push_back(pos - 1);
            }

            if pos < a.len() - 1 && a[pos + 1] == '.' {
                a[pos + 1] = '@';
                deque.push_back(pos + 1);
            }
        }
    }

    for i in 0..n {
        print!("{}", a[i]);
    }
}
