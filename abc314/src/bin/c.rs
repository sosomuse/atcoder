use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        chars: Chars,
        c: [usize; n],
    }

    let mut positions = vec![VecDeque::new(); m];

    for i in 0..n {
        positions[c[i] - 1].push_back(chars[i]);
    }

    for color_positions in positions.iter_mut() {
        let p = color_positions.pop_back().unwrap();
        color_positions.push_front(p);
    }

    for i in 0..n {
        let p = c[i] - 1;
        let c = positions[p].pop_front().unwrap();

        print!("{}", c);
    }
}
