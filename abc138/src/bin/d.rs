use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n-1],
        px: [(Usize1, usize); q],
    };

    let mut graph = vec![vec![]; n];
    let mut counts = HashMap::new();

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for (p, x) in px {
        *counts.entry(p).or_insert(0) += x;
    }

    let mut visited = vec![false; n];
    let mut ans = vec![0; n];

    dfs(0, &graph, &mut visited, &counts, 0, &mut ans);

    for v in ans {
        print!("{} ", v);
    }
}

fn dfs(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    counts: &HashMap<usize, usize>,
    count: usize,
    ans: &mut Vec<usize>,
) {
    if visited[v] {
        return;
    }
    visited[v] = true;

    let current_count = count + counts.get(&v).unwrap_or(&0);

    ans[v] += current_count;

    for &w in graph[v].iter() {
        dfs(w, graph, visited, counts, current_count, ans);
    }
}
