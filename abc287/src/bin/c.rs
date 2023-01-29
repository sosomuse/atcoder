use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];
    let mut count = vec![0; n];

    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);

        count[u] += 1;
        count[v] += 1;
    }

    if let Some(start) = count.iter().position(|&c| c == 1) {
        let paths = dfs(start, &graph);

        if paths.len() == n {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
        return;
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans: Vec<usize> = vec![];
    dfs_inner(v, graph, &mut visited, &mut ans, 1);
    ans
}

fn dfs_inner(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    ans: &mut Vec<usize>,
    count: usize,
) {
    visited[v] = true;
    ans.push(v);

    let nexts = &graph[v];

    if count == 1 || count == graph.len() {
        if nexts.len() != 1 {
            println!("No");
            std::process::exit(0);
        }
    } else {
        if nexts.len() != 2 {
            println!("No");
            std::process::exit(0);
        }
    }

    for &w in nexts.iter() {
        if !visited[w] {
            dfs_inner(w, graph, visited, ans, count + 1);
        }
    }
}
