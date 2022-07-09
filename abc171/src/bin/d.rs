use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    };

    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;

    for (k, v) in &map {
        ans += v * k;
    }

    for _ in 0..q {
        input! {
            b: usize,
            c: usize,
        };

        let count = *map.get(&b).unwrap_or(&0);
        *map.entry(b).or_insert(0) = 0;
        *map.entry(c).or_insert(0) += count;
        ans -= count * b;
        ans += count * c;

        println!("{}", ans);
    }
}
