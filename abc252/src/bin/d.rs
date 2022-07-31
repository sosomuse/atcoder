use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut cnt = HashMap::new();

    for a in a.iter() {
        *cnt.entry(a).or_insert(0) += 1;
    }

    if cnt.len() < 3 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let mut before = 0;
    let mut after = n;

    for c in cnt.values() {
        after -= c;
        ans += before * c * after;
        before += c;
    }

    println!("{}", ans);
}
