use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = n;
    let mut i = 2;
    let mut visited = HashMap::new();

    while i * i <= n {
        if visited.contains_key(&i) {
            i += 1;
            continue;
        }

        let mut j = 2;
        while i.pow(j) <= n {
            visited.insert(i.pow(j), true);
            ans -= 1;
            j += 1;
        }
        i += 1;
    }

    println!("{}", ans);
}
