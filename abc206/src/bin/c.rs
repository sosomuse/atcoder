use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut count = HashMap::new();

    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;

    for i in 0..n {
        ans += n - i - count[&a[i]];
        *count.entry(a[i]).or_insert(0) -= 1;
    }

    println!("{}", ans);
}
