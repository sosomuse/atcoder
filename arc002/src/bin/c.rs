use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: Chars,
    };

    let mut ans = n;
    let mut prev = c[0];
    let s = c.iter().collect::<String>();

    for i in 1..n {
        let cur = format!("{}{}", prev, c[i]);
        let mut s = s.clone();

        while s.contains(&cur) {
            s = s.replace(&cur, "L");
        }

        let mut prev2 = None;
        let mut counts = HashMap::new();

        for char in s.chars() {
            if char == 'L' {
                prev2 = None;
                continue;
            }

            if prev2 == None {
                prev2 = Some(char);
            } else {
                let cur = format!("{}{}", prev2.unwrap(), char);
                *counts.entry(cur).or_insert(0) += 1;
                prev2 = Some(char);
            }
        }

        let max = counts.values().max().unwrap_or(&0);
        ans = ans.min(s.len() - max);

        prev = c[i];
    }

    println!("{}", ans);
}
