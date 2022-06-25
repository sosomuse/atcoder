use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for (a, b) in ab.iter() {
        graph[*a].push(*b);
        graph[*b].push(*a);
    }

    let ans = dfs(ab[0].0, &graph);

    if ans.len() % 2 == 1 {
        println!("No");
        return;
    }

    println!("Yes");
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut ans: Vec<usize> = vec![];
    let mut cycles: Vec<usize> = vec![];
    dfs_inner(v, graph, &mut visited, &mut ans, &mut cycles);
    cycles
}

fn dfs_inner(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    ans: &mut Vec<usize>,
    cycles: &mut Vec<usize>,
) {
    visited[v] = true;
    ans.push(v);

    for &w in graph[v].iter() {
        if visited[w] {
            cycles.push(w);
        }
        if !visited[w] {
            dfs_inner(w, graph, visited, ans, cycles);
        }
    }
}
