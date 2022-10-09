use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut vec = vec![HashSet::new(); n];

    for _ in 0..m {
        input! {
            k: usize,
            s: [usize; k],
        };

        for i in 0..k {
            let x = s[i] - 1;

            for j in 0..k {
                if i == j {
                    continue;
                }

                let y = s[j] - 1;
                vec[x].insert(y);
                vec[y].insert(x);
            }
        }
    }

    let count = vec[0].len();

    for i in 0..vec.len() {
        if count != vec[i].len() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
