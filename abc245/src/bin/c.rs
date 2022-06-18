use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut vec: Vec<HashSet<usize>> = vec![std::collections::HashSet::new(); n];

    for i in 0..n {
        let x = a[n - i - 1];
        let y = b[n - i - 1];

        if i == 0 {
            vec[i].insert(x);
            vec[i].insert(y);
        } else {
            let cp = vec[i - 1].clone();
            let prev = cp;

            for v in prev {
                let is_x = (x as isize - v as isize).abs() <= k;
                let is_y = (y as isize - v as isize).abs() <= k;

                if is_x {
                    vec[i].insert(x);
                }

                if is_y {
                    vec[i].insert(y);
                }
            }

            if vec[i].len() == 0 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
