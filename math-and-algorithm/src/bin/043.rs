use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans = vec![];

    for (a, b) in ab {
        graph[a].push(b);
    }

    dfs(1, &graph, &mut visited, &mut ans);

    if ans.len() == n {
        println!("The graph is connected.");
    } else {
        println!(" The graph is not connected.");
    }
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    visited[v] = true;
    ans.push(v);

    for &next in g[v].iter() {
        if !visited[next] {
            dfs(next, g, visited, ans);
        }
    }
}
