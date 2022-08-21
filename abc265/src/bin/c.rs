use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        // U, D, L, R
        g: [Chars; h],
    };

    let mut x = 1;
    let mut y = 1;

    let mut xy = HashSet::<String>::new();

    loop {
        let current = g[y - 1][x - 1];

        let f = format!("{} {}", x, y);

        if xy.contains(&f) {
            println!("-1");
            return;
        }

        xy.insert(f);

        match current {
            'U' => {
                if y == 1 {
                    println!("{} {}", y, x);
                    return;
                }
                y -= 1;
            }
            'D' => {
                if y == h {
                    println!("{} {}", y, x);
                    return;
                }
                y += 1;
            }
            'L' => {
                if x == 1 {
                    println!("{} {}", y, x);
                    return;
                }
                x -= 1;
            }
            'R' => {
                if x == w {
                    println!("{} {}", y, x);
                    return;
                }
                x += 1;
            }
            _ => unreachable!(),
        }
    }
}
