use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };

    let mut map = HashMap::<String, usize>::new();

    for i in 0..n {
        let v = &mut s[i];
        v.sort();

        let sorted = v.iter().collect::<String>();

        *map.entry(sorted).or_insert(0) += 1;
    }

    let mut ans = 0;

    for (_, v) in map.iter() {
        if *v > 1 {
            ans += (v - 1) * v / 2;
        }
    }

    println!("{}", ans);
}
