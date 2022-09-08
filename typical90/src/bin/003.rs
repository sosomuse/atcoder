use std::collections::VecDeque;

use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in e {
        graph[a].push(b);
        graph[b].push(a);
    }

    let ans = bfs(0, &graph);
    let max = ans.iter().max().unwrap();
    let max_i = ans.iter().position(|&x| x == *max).unwrap();
    let ans = bfs(max_i as usize, &graph);
    let max = ans.iter().max().unwrap();

    println!("{}", max + 1)
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
