use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut a = vec![0; m];
    let mut b = vec![0; m];
    let mut dp = vec![usize::MAX; 1 << n];

    for i in 0..m {
        input! {
            av: usize,
            bv: usize,
        };

        a.push(av);
        b.push(bv);

        input! {
            cv: [usize; bv],
        }

        c[i] = cv;
    }

    let mut ans = usize::max_value();

    for bit in 0..1 << n {
        let mut keys = vec![];

        for i in 0..n {
            if bit >> i & 1 != 0 {
                keys.push(i);
            }
        }

        let mut s = HashSet::new();
        for k in keys.iter() {
            for v in c[*k].iter() {
                s.insert(*v);
            }
        }

        if s.len() == n {
            let sum = keys.iter().fold(0, |acc, x| acc + a[*x]);

            ans = std::cmp::min(ans, sum);
        }
    }

    if ans == usize::max_value() {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
