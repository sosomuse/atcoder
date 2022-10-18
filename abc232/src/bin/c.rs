use std::{collections::VecDeque, vec};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    };

    let mut graph_a = vec![vec![]; n + 1];
    let mut graph_b = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph_a[a].push(b);
        graph_a[b].push(a);
    }

    for (c, d) in cd {
        graph_b[c].push(d);
        graph_b[d].push(c);
    }

    let mut dist: Vec<String> = vec![];

    for i in 0..n {
        let mut dist_a = bfs(i + 1, &graph_a);
        dist_a.sort();

        dist.push(
            dist_a
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    }

    for i in 0..n {
        let mut dist_b = bfs(i + 1, &graph_b);
        dist_b.sort();

        let str_b = dist_b
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",");

        if !dist.contains(&str_b) {
            println!("No");
            return;
        }
    }

    println!("Yes")
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
