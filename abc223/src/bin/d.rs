use proconio::{fastout, input};
use std::{cmp::Reverse, collections::BinaryHeap};

// トポロジカルソート
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    let mut in_deg = vec![0; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        in_deg[b] += 1;
    }

    let mut b = BinaryHeap::new();

    for i in 1..=n {
        if in_deg[i] == 0 {
            b.push(Reverse(i));
        }
    }
    let mut r = vec![];

    while let Some(i) = b.pop() {
        r.push(i);
        for &j in &graph[i.0] {
            in_deg[j] -= 1;
            if in_deg[j] == 0 {
                b.push(Reverse(j));
            }
        }
    }

    if r.len() == n {
        for i in r {
            println!("{} ", i.0);
        }
    } else {
        println!("-1");
    }
}
