use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };

    if h % 3 == 0 || w % 3 == 0 {
        println!("0");
        return;
    }

    fn sub(a: usize, b: usize) -> usize {
        if a % 2 == 0 {
            return a / 2;
        }

        let d = b / 3 + (b + 2) / 3;

        if d >= a {
            return a;
        }

        return (a - d) / 2 + d;
    }

    let ans = min(sub(h, w), sub(w, h));

    println!("{}", ans);
}
