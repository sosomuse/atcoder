use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    for i in 0..w {
        let mut v1 = String::new();
        let mut v2 = String::new();

        for j in 0..h {
            v1.push(s[j][i]);
            v2.push(t[j][i]);
        }

        *map_s.entry(v1).or_insert(0) += 1;
        *map_t.entry(v2).or_insert(0) += 1;
    }

    for (v, i) in map_s {
        if map_t.get(&v) != Some(&i) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
