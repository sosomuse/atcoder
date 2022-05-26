use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        (n, q): (u32, u32),
        x: [u32; q],
    }

    let mut hash_map: HashMap<u32, u32> = std::collections::HashMap::new();
    for i in 1..=n {
        hash_map.entry(i).or_insert(i);
    }

    dbg!(&x);

    for i in 0..x.len() {
        let v = x[i];

        let c = hash_map.clone();
        let c2 = hash_map.clone();
        let current = c.get(&v).unwrap();

        let t = {
            if current == &n {
                current - 1
            } else {
                current + 1
            }
        };

        dbg!(&v);
        dbg!(&t);
        dbg!(&hash_map);
        let (_, next) = c2.into_iter().find(|(_, k)| k == &t).unwrap();

        hash_map.insert(v, next);
        hash_map.insert(next, *current);
    }

    for i in 1..=n {
        let ans = hash_map.get(&i).unwrap();
        print!("{}", ans);

        if i != n {
            print!(" ");
        }
    }
}
