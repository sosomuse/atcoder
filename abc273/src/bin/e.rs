use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut current = VecDeque::new();
    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
            t: String,
        };

        match &*t {
            "ADD" => {
                input! {
                    x: usize,
                };

                current.push_back(x);
            }
            "DELETE" => {
                current.pop_back();
            }
            "SAVE" => {
                input! {
                    x: usize,
                };

                map.insert(x, current.clone());
            }
            "LOAD" => {
                input! {
                    x: usize,
                };

                if let Some(v) = map.get(&x) {
                    current = v.clone();
                } else {
                    current = VecDeque::new();
                }
            }
            _ => todo!(),
        }

        if let Some(v) = current.back() {
            print!("{} ", v);
        } else {
            print!("-1 ");
        }
    }
}
