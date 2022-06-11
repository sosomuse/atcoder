use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let mut ans = 0;

    for bit in 0..1 << n {
        let mut count = 0;

        let mut map = HashMap::<char, usize>::new();

        for i in 0..n {
            if bit & (1 << i) == 0 {
                continue;
            }

            for x in s[i].chars() {
                *map.entry(x).or_insert(0) += 1;
            }
        }

        for (_, v) in map.iter() {
            if *v == k {
                count += 1;
            }
        }

        ans = ans.max(count);
    }

    println!("{}", ans);
}
