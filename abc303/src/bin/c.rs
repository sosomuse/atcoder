use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    input! {
        _: usize,
        m: usize,
        mut h: i64,
        k: i64,
        s: Chars,
        items: [(i32, i32); m],
    }

    let mut item_map = HashMap::new();
    for (x, y) in items {
        item_map.insert(Point { x, y }, true);
    }

    let mut position = Point { x: 0, y: 0 };

    for si in s {
        match si {
            'R' => position.x += 1,
            'L' => position.x -= 1,
            'U' => position.y += 1,
            'D' => position.y -= 1,
            _ => panic!("Invalid input"),
        }

        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }

        if h < k && item_map.contains_key(&position) {
            h = k;
            item_map.remove(&position);
        }
    }

    println!("Yes");
}
