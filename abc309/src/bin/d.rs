use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let n = n1 + n2;
    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }

    let dist1 = bfs(0, &g);
    let dist2 = bfs(n - 1, &g);

    let dist1_max = dist1.iter().max().unwrap();
    let dist2_max = dist2.iter().max().unwrap();

    println!("{}", dist1_max + dist2_max + 1);
}

fn bfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<isize> {
    let mut dist: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<usize> = std::collections::VecDeque::new();
    queue.push_front(v);
    dist[v] = 0;

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();
        queue.pop_front().unwrap();

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            if dist[nex] == -1 {
                dist[nex] = dist[pos] + 1;
                queue.push_back(nex);
            }
        }
    }

    dist
}
