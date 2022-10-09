use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: String,
        };

        match t {
            1 => {
                input! {
                    y: usize,
                };

                map.insert(x, y);
            }
            2 => {
                if let Some(&y) = map.get(&x) {
                    println!("{}", y);
                }
            }
            _ => unreachable!(),
        }
    }
}
