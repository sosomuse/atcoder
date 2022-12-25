use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut queue = VecDeque::new();
    let mut set = HashSet::new();

    for i in 0..s.len() {
        let v = s[i];
        if v == '(' {
            queue.push_back(HashSet::new());
            continue;
        }
        if v == ')' {
            let t = queue.pop_back().unwrap();

            for v2 in t {
                set.remove(&v2);
            }
            continue;
        }

        if let Some(t) = queue.back_mut() {
            t.insert(v);
        }

        if set.contains(&v) {
            println!("No");
            return;
        }
        set.insert(v);
    }

    println!("Yes");
}
