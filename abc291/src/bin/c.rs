use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    };

    let mut set = HashSet::new();
    let mut cx = 0;
    let mut cy = 0;

    set.insert((cx, cy));

    for c in s {
        match c {
            'L' => cx -= 1,
            'R' => cx += 1,
            'U' => cy += 1,
            'D' => cy -= 1,
            _ => unreachable!(),
        }

        if set.contains(&(cx, cy)) {
            println!("Yes");
            return;
        }

        set.insert((cx, cy));
    }

    println!("No");
}
