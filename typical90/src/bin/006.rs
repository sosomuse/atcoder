use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let s_with_index = s
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c as u8, i))
        .collect::<Vec<_>>();

    let mut set = BTreeSet::new();

    for i in 0..n - k {
        set.insert(s_with_index[i]);
    }

    let mut ans = vec![];
    let mut last_i = 0;

    for i in 0..k {
        set.insert(s_with_index[i + n - k]);

        let &(c, j) = set.iter().next().unwrap();

        for k in last_i..j {
            for u in 'a' as u8..='z' as u8 {
                set.remove(&(u, k));
            }
        }

        set.remove(&(c, j));

        ans.push(c as char);
        last_i = j + 1;
    }

    println!("{}", ans.into_iter().collect::<String>());
}
