use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for g in graph.iter_mut() {
        g.sort();
    }

    dfs(1, &graph);
}

// オイラーツアー
fn dfs(v: usize, graph: &Vec<Vec<usize>>) -> () {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    dfs_inner(v, graph, &mut visited, 0);
}

fn dfs_inner(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, prev: usize) {
    visited[v] = true;
    print!("{} ", v);

    for &w in graph[v].iter() {
        if w != prev {
            dfs_inner(w, graph, visited, v);
            print!("{} ", v);
        }
    }
}
