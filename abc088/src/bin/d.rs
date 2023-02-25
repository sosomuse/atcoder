use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let graph = create_maze_graph(h, w, &s, '#');
    let dist = bfs(0, &graph);
    let goal = h * w - 1;

    if dist[goal] == -1 {
        println!("-1");
        return;
    }

    let mut white_count = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                white_count += 1;
            }
        }
    }

    println!("{}", white_count - dist[goal] - 1);
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
