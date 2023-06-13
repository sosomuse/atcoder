use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut t = VecDeque::new();
    let mut is_reversible = false;

    for c in s {
        if c == 'R' {
            is_reversible = !is_reversible;
            continue;
        }

        if is_reversible {
            if t.front() == Some(&c) {
                t.pop_front();
            } else {
                t.push_front(c);
            }
        } else {
            if t.back() == Some(&c) {
                t.pop_back();
            } else {
                t.push_back(c);
            }
        }
    }

    let t: String = if is_reversible {
        t.into_iter().rev().collect()
    } else {
        t.into_iter().collect()
    };

    println!("{}", t);
}
