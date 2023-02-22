use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    };

    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(v[i]);
    }

    if set.len() == 1 {
        println!("{}", n / 2);
        return;
    }

    let mut odd = HashMap::new();
    let mut even = HashMap::new();
    for (i, &x) in v.iter().enumerate() {
        if i % 2 == 0 {
            even.entry(x).and_modify(|e| *e += 1).or_insert(1);
        } else {
            odd.entry(x).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let odd_max = odd.iter().max_by_key(|&(_, v)| v).unwrap();
    let even_max = even.iter().max_by_key(|&(_, v)| v).unwrap();

    if odd_max.0 != even_max.0 {
        println!("{}", n - odd_max.1 - even_max.1);
    } else {
        let mut ans = std::u32::MAX as usize;
        let odd_next_max = odd
            .iter()
            .max_by_key(|&(&k, v)| if k == *odd_max.0 { 0 } else { *v });
        let even_next_max = even
            .iter()
            .max_by_key(|&(&k, v)| if k == *even_max.0 { 0 } else { *v });

        if let Some((v, &c)) = odd_next_max {
            if odd_max.0 != v {
                ans = ans.min(n - even_max.1 - c);
            }
        }

        if let Some((v, &c)) = even_next_max {
            if even_max.0 != v {
                ans = ans.min(n - odd_max.1 - c);
            }
        }

        println!("{}", ans);
    }
}
