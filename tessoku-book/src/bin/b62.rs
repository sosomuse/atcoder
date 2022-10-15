use std::{collections::VecDeque, vec};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n + 1];
    let mut ans: VecDeque<usize> = VecDeque::new();
    let mut end = false;

    // dbg!(&graph);

    dfs(&graph, &mut ans, &mut visited, 1, n, &mut end);

    for i in ans {
        print!("{} ", i);
    }
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    ans: &mut VecDeque<usize>,
    visited: &mut Vec<bool>,
    s: usize,
    n: usize,
    end: &mut bool,
) -> () {
    visited[s] = true;
    ans.push_back(s);

    if s == n {
        *end = true;
        return;
    }

    if *end {
        return;
    }

    for v in graph[s].iter() {
        if !visited[*v] && !*end {
            dfs(graph, ans, visited, *v, n, end);

            if !*end {
                ans.pop_back();
            }
        }
    }
}
