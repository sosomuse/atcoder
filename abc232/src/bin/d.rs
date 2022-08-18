use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut graph: Vec<Vec<usize>> = vec![vec![]; h * w];

    for i in 0..h {
        for j in 0..w {
            let v = c[i][j];

            if v == '#' {
                continue;
            }

            if i + 1 < h && c[i + 1][j] != '#' {
                graph[i * w + j].push((i + 1) * w + j);
            }

            if j + 1 < c[i].len() && c[i][j + 1] != '#' {
                graph[i * w + j].push(i * w + j + 1);
            }
        }
    }

    let ans = bfs(0, &graph);

    println!("{}", ans.iter().max().unwrap() + 1);
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
