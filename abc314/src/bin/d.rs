use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    };

    let mut upper_flag = true;
    let mut uppers = HashSet::new();
    for i in 0..n {
        uppers.insert(i);
    }

    for _ in 0..q {
        input! {
            operation: (u32, usize, char),
        };

        let (t, x, c) = operation;

        if t == 1 {
            uppers.insert(x - 1);
            s[x - 1] = c;
        } else if t == 2 {
            upper_flag = false;
            uppers = HashSet::new();
        } else if t == 3 {
            upper_flag = true;
            uppers = HashSet::new();
        }
    }

    for i in 0..n {
        if uppers.contains(&i) {
            print!("{}", s[i]);
        } else {
            if upper_flag {
                print!("{}", s[i].to_uppercase());
            } else {
                print!("{}", s[i].to_lowercase());
            }
        }
    }
}
