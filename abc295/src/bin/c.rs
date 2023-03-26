use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut colors = HashMap::new();
    for i in a {
        *colors.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, v) in colors {
        ans += v / 2;
    }

    println!("{}", ans);
}
