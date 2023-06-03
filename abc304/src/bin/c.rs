use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        d: isize,
        xy: [(isize, isize); n],
    }
    let d2 = d * d;
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            if (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2) <= d2 {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let mut seen = vec![false; n];
    let mut q = VecDeque::new();
    seen[0] = true;
    q.push_back(0);
    while let Some(v) = q.pop_front() {
        for &w in &g[v] {
            if !seen[w] {
                seen[w] = true;
                q.push_back(w);
            }
        }
    }
    for i in 0..n {
        if seen[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
