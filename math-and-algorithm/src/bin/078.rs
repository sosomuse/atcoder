use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; m];

    for i in 0..m {
        let (a, b) = ab[i];
        graph[a].push(b);
        graph[b].push(a);
    }

    let ans = bfs(1, &graph);

    for i in 1..m {
        let mut v = ans[i];
        v = v.min(120).max(0);
        println!("{}", v);
    }
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
