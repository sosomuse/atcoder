use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut set_a = BTreeSet::new();
    let mut set_b = BTreeSet::new();

    for i in 1..=n {
        set_a.insert(i);
    }

    for _ in 0..q {
        input! {
            t: usize,
        };

        match t {
            1 => {
                let min = *set_a.iter().next().unwrap();
                set_a.remove(&min);
                set_b.insert(min);
            }
            2 => {
                input! {
                    x: usize,
                };
                set_b.remove(&x);
            }
            3 => {
                println!("{}", set_b.iter().next().unwrap());
            }
            _ => unimplemented!(),
        }
    }
}
