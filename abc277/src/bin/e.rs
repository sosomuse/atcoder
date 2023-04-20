use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

// 01-BFS
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
            adj_list[u].push(v);
            adj_list[v].push(u);
        } else {
            adj_list[u + n].push(v + n);
            adj_list[v + n].push(u + n);
        }
    }

    for &s in &switches {
        adj_list[s].push(s + n);
        adj_list[s + n].push(s);
    }

    let mut dist = vec![std::usize::MAX; 2 * n];
    let mut queue = VecDeque::new();
    dist[0] = 0;
    queue.push_back(0);

    while let Some(u) = queue.pop_front() {
        for &v in &adj_list[u] {
            if dist[v] != std::usize::MAX {
                continue;
            }

            if (u < n && u + n == v) || (u >= n && u - n == v) {
                dist[v] = dist[u];
                queue.push_front(v);
            } else {
                dist[v] = dist[u] + 1;
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
