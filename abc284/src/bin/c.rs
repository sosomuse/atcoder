use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n];

    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    let mut visited = vec![false; n];
    let mut ans = 0;

    for v in 0..n {
        if !visited[v] {
            dfs(v, &graph, &mut visited);
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[v] = true;

    for &w in graph[v].iter() {
        if !visited[w] {
            dfs(w, graph, visited);
        }
    }
}
