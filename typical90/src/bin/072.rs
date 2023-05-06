use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let graph = create_maze_graph(h, w, &c, '#');

    for i in 0..h {
        for j in 0..w {
            let p = w * i + j;
            let ans = bfs(p, &graph);
            println!("{:?}", ans);
        }
    }
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<isize> {
    let mut dist: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<usize> = std::collections::VecDeque::new();
    let mut max = 0;
    queue.push_front(start);

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();
        queue.pop_front().unwrap();

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            if nex == start {
                max = std::cmp::max(max, dist[pos] + 1);
                continue;
            }

            if dist[nex] == -1 {
                dist[nex] = dist[pos] + 1;
                queue.push_back(nex);
            }
        }
    }

    dist
}

fn create_maze_graph(h: usize, w: usize, s: &Vec<Vec<char>>, wall: char) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; h * w];

    for i in 0..h {
        let v = &s[i];
        for j in 0..w {
            let ch = v[j];
            if ch == wall {
                continue;
            }

            let p = w * i + j;
            // up
            if i > 0 && s[i - 1][j] != wall {
                graph[p].push(w * (i - 1) + j);
            }
            // down
            if i != (h - 1) && s[i + 1][j] != wall {
                graph[p].push(w * (i + 1) + j);
            }
            // left
            if j > 0 && s[i][j - 1] != wall {
                graph[p].push(p - 1);
            }
            // right
            if j != (w - 1) && s[i][j + 1] != wall {
                graph[p].push(p + 1);
            }
        }
    }

    graph
}
