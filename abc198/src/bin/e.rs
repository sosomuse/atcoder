use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(Usize1, Usize1); n - 1],
    };

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ans = dfs(0, &graph, &c);
    ans.sort();

    for a in ans {
        println!("{}", a);
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, colors: &Vec<usize>) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut counts = HashMap::new();
    let mut ans = vec![];
    dfs_inner(v, graph, colors, &mut visited, &mut counts, &mut ans);
    ans
}

fn dfs_inner(
    v: usize,
    graph: &Vec<Vec<usize>>,
    colors: &Vec<usize>,
    visited: &mut Vec<bool>,
    counts: &mut HashMap<usize, usize>,
    ans: &mut Vec<usize>,
) {
    visited[v] = true;
    *counts.entry(colors[v]).or_insert(0) += 1;

    if counts[&colors[v]] == 1 {
        ans.push(v + 1);
    }

    for &w in graph[v].iter() {
        if !visited[w] {
            dfs_inner(w, graph, colors, visited, counts, ans);
        }
    }

    *counts.entry(colors[v]).or_insert(0) -= 1;
}
