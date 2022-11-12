use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    // 袋の中
    let mut set = BTreeSet::<isize>::new();
    // 今まで加えられた数の合計
    let mut count = 0;
    let mut map = std::collections::HashMap::<isize, usize>::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: isize,
            }

            let v = x - count;

            set.insert(v);
            *map.entry(v).or_insert(0) += 1;
        } else if t == 2 {
            input! {
                x: isize,
            }

            count += x;
        } else {
            let mut v = 0;

            if let Some(x) = set.iter().next() {
                v = *x;
            }

            println!("{}", v + count);

            if let Some(y) = map.get_mut(&v) {
                if *y > 0 {
                    *y -= 1;
                }

                if *y == 0 {
                    set.remove(&v);
                }
            }
        }
    }
}
