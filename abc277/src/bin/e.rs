use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1, usize); m],
        switches: [Usize1; k],
    }

    let mut adj_list = vec![vec![]; 2 * n];
    for (u, v, a) in edges {
        if a == 1 {
            adj_list[u].push((v, 1));
            adj_list[v].push((u, 1));
        } else {
            adj_list[u + n].push((v + n, 1));
            adj_list[v + n].push((u + n, 1));
        }
    }

    for &s in &switches {
        adj_list[s].push((s + n, 0));
        adj_list[s + n].push((s, 0));
    }

    let mut dist = vec![std::usize::MAX; 2 * n];
    let mut queue = VecDeque::new();
    dist[0] = 0;
    queue.push_back(0);

    while let Some(u) = queue.pop_front() {
        for &(v, cost) in &adj_list[u] {
            if dist[v] == std::usize::MAX {
                dist[v] = dist[u] + cost;
                queue.push_back(v);
            }
        }
    }

    let ans = dist[n - 1].min(dist[2 * n - 1]);
    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
