use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: isize,
    }

    let mut map = BTreeMap::<isize, isize>::new();

    for _ in 0..q {
        input! {
            cmd: u8,
        }

        match cmd {
            1 => {
                input! {
                    x: isize,
                }

                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: isize,
                    c: isize
                }

                let count = map.get(&x).unwrap_or(&0);
                let delete_count = c.min(*count);
                if count - delete_count > 0 {
                    *map.entry(x).or_insert(0) -= delete_count;
                } else {
                    map.remove(&x);
                }
            }
            3 => {
                let min = map.iter().next().unwrap().0;
                let max = map.iter().next_back().unwrap().0;
                println!("{}", max - min);
            }
            _ => {
                //
            }
        }
    }
}
