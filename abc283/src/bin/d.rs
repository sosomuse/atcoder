use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut vec: Vec<HashSet<char>> = vec![HashSet::new(); 100];
    let mut count = 0;

    for c in s {
        match c {
            '(' => {
                count += 1;
                vec[count] = vec[count - 1].clone();
                vec.push(HashSet::new());
            }
            ')' => {
                vec[count].clear();
                count -= 1
            }
            other => {
                if vec[count].contains(&other) {
                    println!("No");
                    return;
                }

                vec[count].insert(other);
            }
        }
    }

    println!("Yes");
}
