use proconio::input;

const MAX: usize = 1000000;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];

    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 0_usize;
    let mut visited = vec![false; n + 1];

    dfs(1, &graph, &mut visited, &mut ans);

    println!("{}", ans);
}

// パス数の数え上げ
fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut usize) {
    if *ans == MAX {
        return;
    }

    visited[v] = true;
    *ans += 1;

    for &w in graph[v].iter() {
        if !visited[w] {
            dfs(w, graph, visited, ans);
        }
    }

    visited[v] = false;
}
