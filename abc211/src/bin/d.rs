use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dist = bfs(1, &graph);
    let mut dp = vec![0usize; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    dp[n] = 1;
    queue.push_back(n);

    while let Some(v) = queue.pop_front() {
        for i in 0..graph[v].len() {
            let nex = graph[v][i];
            if dist[nex] == dist[v] - 1 {
                if dp[nex] == 0 {
                    queue.push_back(nex);
                }

                dp[nex] += dp[v];
                dp[nex] %= 1000000007;
            }
        }
    }

    println!("{}", dp[1])
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
