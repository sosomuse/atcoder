use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
    }

    let mut ans = 0;

    for i in 0..n {
        let mut visited = vec![false; n];
        let mut count = 0;
        dfs(i, &edges, &mut visited, &mut count);
        ans += count;
    }

    println!("{}", ans);
}

fn dfs(v: usize, edges: &Vec<Vec<usize>>, visited: &mut Vec<bool>, count: &mut usize) {
    visited[v] = true;
    *count += 1;

    for &next_v in &edges[v] {
        if visited[next_v] {
            continue;
        }
        dfs(next_v, edges, visited, count);
    }
}
