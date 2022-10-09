use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    let mut ans: usize = 0;

    for v in a {
        if let Some(x) = map.get(&v) {
            ans += x;
        }

        *map.entry(v).or_insert(0) += 1;
    }

    println!("{}", ans);
}
