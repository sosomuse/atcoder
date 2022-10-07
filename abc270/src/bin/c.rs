use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        uv: [(usize, usize); n-1],
    };

    let mut graph = vec![vec![]; n + 1];

    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let ans = dfs(x, &graph, y);

    for i in ans {
        println!("{}", i);
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, y: usize) -> VecDeque<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans: VecDeque<usize> = VecDeque::new();
    let mut stop = false;
    dfs_inner(v, graph, &mut visited, &mut ans, y, &mut stop);
    ans
}

fn dfs_inner(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    ans: &mut VecDeque<usize>,
    t: usize,
    stop: &mut bool,
) {
    if *stop {
        return;
    }

    visited[v] = true;
    ans.push_back(v);

    if v == t {
        *stop = true;
        return;
    }

    let next = &graph[v];

    for w in next {
        if !visited[*w] {
            dfs_inner(*w, graph, visited, ans, t, stop);
        }
    }

    if !*stop {
        ans.pop_back();
        return;
    }
}
