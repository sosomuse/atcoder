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

fn dfs(v: usize, graph: &Vec<Vec<usize>>, t: usize) -> VecDeque<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans: VecDeque<usize> = VecDeque::new();
    dfs_inner(v, graph, &mut visited, &mut ans, t, 0);
    ans
}

fn dfs_inner(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    ans: &mut VecDeque<usize>,
    t: usize,
    c: usize,
) {
    visited[v] = true;
    ans.push_back(v);

    if v == t {
        return;
    }

    let next = &graph[v];

    if ans.len() != 1 && next.len() == 1 {
        for _ in 0..c {
            ans.pop_back();
        }

        return;
    }

    for &w in next {
        if !visited[w] {
            dfs_inner(
                w,
                graph,
                visited,
                ans,
                t,
                if next.len() > 2 {
                    1
                } else if next.len() == 2 {
                    c + 1
                } else {
                    c
                },
            );
        }
    }
}
