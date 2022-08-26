use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut set = HashSet::<String>::new();

    for i in 0..n {
        if set.contains(&s[i]) {
            continue;
        }

        set.insert(s[i].to_string());
        println!("{}", i + 1);
    }
}
