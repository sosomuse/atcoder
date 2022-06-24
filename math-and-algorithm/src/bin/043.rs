use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans: Vec<usize> = vec![];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    dfs(1, &graph, &mut visited, &mut ans);

    if ans.len() == n {
        println!("The graph is connected");
    } else {
        println!("The graph is not connected");
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    visited[v] = true;
    ans.push(v);

    for &w in graph[v].iter() {
        if !visited[w] {
            dfs(w, graph, visited, ans);
        }
    }
}
