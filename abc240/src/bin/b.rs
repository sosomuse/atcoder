use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut set = HashSet::<usize>::new();

    for v in a {
        set.insert(v);
    }

    println!("{}", set.len());
}
