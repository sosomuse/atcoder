use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }

    let mut set = HashSet::new();

    if x == 0 {
        println!("Yes");
        return;
    }

    for &ai in &a {
        if set.contains(&(ai - x)) || set.contains(&(ai + x)) {
            println!("Yes");
            return;
        }
        set.insert(ai);
    }

    println!("No");
}
