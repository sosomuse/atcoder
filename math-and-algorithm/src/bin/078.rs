use std::collections::VecDeque;

use proconio::{input, marker::Isize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Isize1, Isize1); m],
    }

    let mut graph: Vec<Vec<isize>> = vec![vec![]; n];

    for i in 0..m {
        let (a, b) = ab[i];
        graph[a as usize].push(b);
        graph[b as usize].push(a);
    }

    let ans = bfs(0, &graph);

    for mut v in ans {
        if v == -1 {
            v = 120;
        }
        println!("{}", v);
    }
}

fn bfs(v: usize, graph: &Vec<Vec<isize>>) -> Vec<isize> {
    let mut dist: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<usize> = std::collections::VecDeque::new();
    queue.push_front(v);
    dist[v] = 0;

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();
        queue.pop_front().unwrap();

        for &nex in &graph[pos] {
            if dist[nex as usize] == -1 {
                dist[nex as usize] = (dist[pos] + 1).min(120);
                queue.push_back(nex as usize);
            }
        }
    }

    dist
}
