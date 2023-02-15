use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        (r1, c1): (Usize1, Usize1),
        (r2, c2): (Usize1, Usize1),
        s: [Chars; h],
    };

    let graph = create_maze_graph(h, w, s, '#');
    let dist = bfs(r1 * w + c1, &graph, w);

    println!("{}", dist[r2 * w + c2]);
}

fn create_maze_graph(h: usize, w: usize, s: Vec<Vec<char>>, wall: char) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; h * w];

    for i in 0..h {
        let v = &s[i];
        for j in 0..w {
            let ch = v[j];
            if ch == wall {
                continue;
            }

            let p = w * i + j;
            // upper
            if i > 0 && s[i - 1][j] != wall {
                graph[p].push(w * (i - 1) + j);
            }
            // lower
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

fn bfs(v: usize, graph: &Vec<Vec<usize>>, w: usize) -> Vec<isize> {
    let mut dist: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<(usize, char)> = std::collections::VecDeque::new();
    queue.push_front((v, '.'));
    dist[v] = 0;

    while !queue.is_empty() {
        let (pos, direction) = *queue.front().unwrap();
        queue.pop_front().unwrap();

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            let nex_dir = {
                if pos >= w && nex == pos - w {
                    'u'
                } else if nex == pos + w {
                    'd'
                } else if pos > 0 && nex == pos - 1 {
                    'l'
                } else {
                    'r'
                }
            };

            let nex_count = if nex_dir == direction || direction == '.' {
                dist[pos]
            } else {
                dist[pos] + 1
            };

            if dist[nex] == -1 || dist[nex] > nex_count {
                dist[nex] = nex_count;
                queue.push_back((nex, nex_dir));
            }
        }
    }

    dist
}
