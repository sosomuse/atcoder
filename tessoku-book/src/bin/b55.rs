use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        if t == 1 {
            set.insert(x);
        } else if t == 2 {
            let first = set.range(x..).next();
            let last = set.range(..x).rev().next();
            let mut ans = 1000000000000;

            if let Some(first) = first {
                ans = ans.min((x as isize - *first as isize).abs() as usize);
            }

            if let Some(last) = last {
                ans = ans.min((x as isize - *last as isize).abs() as usize);
            }

            if ans == 1000000000000 {
                println!("-1");
            } else {
                println!("{}", ans);
            }
        }
    }
}
