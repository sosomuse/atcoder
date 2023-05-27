use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut adj = vec![HashSet::new(); n];
    for row in &a {
        for j in 0..n - 1 {
            let u = row[j] - 1;
            let v = row[j + 1] - 1;
            adj[u].insert(v);
            adj[v].insert(u);
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if !adj[i].contains(&j) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
