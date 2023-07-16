use proconio::input;
use std::cmp::min;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = HashSet::new();

    for string in s {
        let reversed = string.chars().rev().collect::<String>();
        if set.contains(&string) || set.contains(&reversed) {
            continue;
        }
        set.insert(min(string, reversed));
    }

    println!("{}", set.len());
}
