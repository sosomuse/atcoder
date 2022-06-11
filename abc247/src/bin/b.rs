use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    for i in 0..n {
        let mut set = HashSet::new();
        for j in (0..n).filter(|x| *x != i) {
            set.insert(&st[j].0);
            set.insert(&st[j].1);
        }
        if set.contains(&st[i].0) && set.contains(&st[i].1) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
