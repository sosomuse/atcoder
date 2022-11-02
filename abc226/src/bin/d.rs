use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    // let mut ans = 0;
    let mut set: HashSet<(isize, isize)> = HashSet::new();

    for i in 0..n {
        let (x1, y1) = xy[i];

        for j in 0..n {
            if i == j {
                continue;
            }

            let (x2, y2) = xy[j];

            let (dx, dy) = (x2 - x1, y2 - y1);

            if dx == 0 {
                let y = if dy > 0 { 1 } else { -1 };
                set.insert((0, y));
            } else if dy == 0 {
                let x = if dx > 0 { 1 } else { -1 };
                set.insert((x, 0));
            } else {
                let x = gcd(dx, dy);
                set.insert((dx / x.abs(), dy / x.abs()));
            }
        }
    }

    println!("{}", set.len());
}

// ユークリッドの互除法（最大公約数）
fn gcd(mut a: isize, mut b: isize) -> isize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}
