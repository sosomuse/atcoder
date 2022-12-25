use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut x = VecDeque::new();
    let mut y = vec![];
    let mut z = HashSet::new();

    for i in 0..s.len() {
        let v = s[i];
        if v == '(' {
            x.push_back(HashSet::new());
        } else if v == ')' {
            let t = x.pop_back().unwrap();

            if !x.is_empty() {
                let t2 = x.back_mut().unwrap();

                for v in t.iter() {
                    t2.insert(*v);
                }
            }

            y.push(t);
        } else {
            if !x.is_empty() {
                let t = x.back_mut().unwrap();
                t.insert(v);
            }
        }
    }

    let mut count = 0;

    for i in 0..s.len() {
        let v = s[i];
        if v == '(' {
            continue;
        }

        if v == ')' {
            let t = &y[count];

            for v in t {
                z.remove(v);
            }

            count += 1;
        } else {
            if z.contains(&v) {
                println!("No");
                return;
            }
            z.insert(v);
        }
    }

    println!("Yes");
}
