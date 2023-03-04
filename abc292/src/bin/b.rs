use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        _: usize,
        q: usize,
    };

    let mut count = HashMap::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        match t {
            1 => {
                let c = count.entry(x).or_insert(0);
                *c += 1;
            }
            2 => {
                let c = count.entry(x).or_insert(0);
                *c += 2;
            }
            3 => {
                if let Some(c) = count.get(&x) {
                    if *c >= 2 {
                        println!("Yes");
                        continue;
                    }
                }

                println!("No");
            }
            _ => unreachable!(),
        }
    }
}
