use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    };

    let mut ans = 0;

    for bit in 0..1 << h {
        let mut vec = vec![];
        for i in 0..h {
            if bit & 1 << i != 0 {
                vec.push(i);
            }
        }
        let mut counts = HashMap::new();

        if vec.len() == 0 {
            continue;
        }

        for j in 0..w {
            let is_same = vec.iter().all(|&i| p[i][j] == p[vec[0]][j]);
            if is_same {
                *counts.entry(p[vec[0]][j]).or_insert(0) += vec.len();
            }
        }

        let max_count = counts.values().max().unwrap_or(&0);
        ans = std::cmp::max(ans, *max_count);
    }

    println!("{}", ans);
}
