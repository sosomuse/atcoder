use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        q: usize,
        mut s: Chars,
    };

    let mut queue = VecDeque::new();

    for v in s {
        queue.push_back(v);
    }

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        match t {
            1 => {
                let m = x % queue.len();
                for _ in 0..m {
                    let v = queue.swap_remove_back(queue.len() - 1).unwrap();
                    queue.push_front(v);
                }
                // dbg!(&queue);
            }
            2 => {
                println!("{}", queue[x - 1]);
            }
            _ => unimplemented!(),
        }
    }
}
