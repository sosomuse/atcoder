use std::collections::BTreeSet;

use proconio::input;

const MAX: usize = 1000000010;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    let mut ans: usize = 0;

    for i in 0..n {
        let (x, y) = xy[i];
        xs.insert((x, y));
        ys.insert((y, x));
    }

    for i in 0..n {
        let (x, y) = xy[i];

        let x_list = ys
            .range((y, x + 1)..=(y, MAX))
            .map(|(_, x)| *x)
            .collect::<Vec<_>>();
        let y_list = xs
            .range((x, y + 1)..=(x, MAX))
            .map(|(_, y)| *y)
            .collect::<Vec<_>>();

        for j in 0..x_list.len() {
            for k in 0..y_list.len() {
                let v1 = x_list[j];
                let v2 = y_list[k];
                if xs.contains(&(v1, v2)) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
