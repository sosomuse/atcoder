use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    let mut characters = HashSet::new();
    for (i, c) in s.chars().enumerate() {
        characters.insert(c);
        if characters.len() == 3 {
            println!("{}", i + 1);
            break;
        }
    }
}
