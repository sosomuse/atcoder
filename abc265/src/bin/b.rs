use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: usize,
        a: [usize; n - 1],
        xy: [(usize, usize); m],
    };

    let mut map = HashMap::<usize, usize>::new();

    for &(x, y) in &xy {
        *map.entry(x).or_insert(0) += y;
    }

    for i in 0..n - 1 {
        let v = a[i];
        if v >= t {
            println!("No");
            return;
        }

        t -= v;

        if let Some(bonus) = map.get(&(i + 2)) {
            t += bonus;
        };
    }

    println!("Yes");
}
