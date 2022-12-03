use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let graph = {
        let mut graph = vec![vec![]; n];
        for (a, b) in ab {
            graph[a - 1].push(b - 1);
            graph[b - 1].push(a - 1);
        }
        graph
    };

    let ans = bfs(0, &graph);

    dbg!(ans);
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
