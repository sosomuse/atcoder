use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let n = 1048576;
    let mut set = BTreeSet::new();
    let mut vec: Vec<isize> = vec![-1; n];

    for i in 0..n {
        set.insert(i);
    }

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        let h = x % n;

        if t == 1 {
            let mut range = set.range(h..);
            let i = if let Some(i) = range.next() {
                *i
            } else {
                *set.range(..).next().unwrap()
            };

            vec[i] = x as isize;
            set.remove(&i);
        } else {
            println!("{}", vec[h]);
        }
    }
}
