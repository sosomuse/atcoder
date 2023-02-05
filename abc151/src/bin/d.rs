use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        t: usize,
        c: [Chars; r],
    };

    let mut graph = vec![vec![]; r * t];

    for i in 0..r {
        let v = &c[i];
        for j in 0..t {
            let ch = v[j];

            if ch == '#' {
                continue;
            }

            if i > 0 && c[i - 1][j] == '.' {
                graph[t * i + j].push(t * (i - 1) + j);
            }

            if i != (r - 1) && c[i + 1][j] == '.' {
                graph[t * i + j].push(t * (i + 1) + j);
            }

            if j > 0 && c[i][j - 1] == '.' {
                graph[t * i + j].push(t * i + j - 1);
            }

            if j != (t - 1) && c[i][j + 1] == '.' {
                graph[t * i + j].push(t * i + j + 1);
            }
        }
    }

    let mut ans = 0;

    for i in 0..r * t {
        let dist = bfs(i, &graph);
        ans = std::cmp::max(ans, *dist.iter().max().unwrap());
    }

    println!("{}", ans);
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
