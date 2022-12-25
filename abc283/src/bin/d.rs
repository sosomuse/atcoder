use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut x = VecDeque::new();
    let mut z = HashSet::new();

    for i in 0..s.len() {
        let v = s[i];
        if v == '(' {
            x.push_back(HashSet::new());
            continue;
        }
        if v == ')' {
            let t = x.pop_back().unwrap();

            for v2 in t {
                z.remove(&v2);
            }
            continue;
        }

        if let Some(t) = x.back_mut() {
            t.insert(v);
        }

        if z.contains(&v) {
            println!("No");
            return;
        }
        z.insert(v);
    }

    println!("Yes");
}
