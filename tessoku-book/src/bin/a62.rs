use std::vec;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut ans: Vec<usize> = vec![];

    dfs(&graph, &mut ans, &mut visited, 0);

    if ans.len() == n {
        println!("The graph is connected.")
    } else {
        println!("The graph is not connected.")
    }
}

fn dfs(graph: &Vec<Vec<usize>>, ans: &mut Vec<usize>, visited: &mut Vec<bool>, s: usize) -> () {
    visited[s] = true;
    ans.push(s);

    for v in graph[s].iter() {
        if !visited[*v] {
            dfs(graph, ans, visited, *v)
        }
    }
}
