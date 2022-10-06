use proconio::input;
use std::collections::{BTreeMap, HashMap};

fn main() {
    input! {
        n: usize,
        d: usize,
        xy: [(usize, usize); n],
    };

    let mut origin = HashMap::<usize, Vec<usize>>::new();
    let mut map = BTreeMap::<usize, usize>::new();
    let mut ans = 0;

    for (x, y) in xy {
        origin.entry(x).or_insert(vec![]).push(y);
    }

    for i in 1..=d {
        if let Some(v) = origin.get(&i) {
            for j in v {
                map.entry(*j).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        let mut remove = 0;

        if let Some((x, y)) = map.iter_mut().max() {
            if *y > 0 {
                ans += x;
                *y -= 1;

                if *y == 0 {
                    remove = *x;
                }
            }
        }

        if remove != 0 {
            map.remove(&remove);
        }
    }

    println!("{}", ans)
}
