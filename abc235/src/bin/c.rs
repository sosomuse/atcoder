use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    let mut map = HashMap::<usize, Vec<usize>>::new();

    for i in 0..n {
        let v = a[i];
        map.entry(v).or_insert(vec![]).push(i + 1);
    }

    for _ in 0..q {
        input! {
            x: usize,
            k: usize,
        };

        let v = &map.get(&x);
        let ans = match v {
            Some(v2) => v2.get(k - 1),
            None => {
                println!("-1");
                continue;
            }
        };

        match ans {
            Some(v) => println!("{}", v),
            None => println!("-1"),
        }
    }
}
