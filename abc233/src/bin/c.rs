use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };

    let mut map = HashMap::<usize, usize>::new();

    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        };

        for &a in a.iter() {
            *map.entry(a).or_insert(0) += 1;
        }
    }

    let mut ans = 0;

    for (k, v) in map.iter() {
        if x % k != 0 {
            continue;
        }

        let m = x / k;

        let n = map.get(&m);

        match n {
            Some(t) => {
                dbg!(k, m, t);
                ans += t * v;
            }
            None => {}
        }
    }

    println!("{}", ans);
}
