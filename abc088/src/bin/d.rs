use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let graph = create_maze_graph(h, w, &s, '#');
    let dist = bfs(h, w, &graph, (0, 0), (h - 1, w - 1));

    let mut white_count = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                white_count += 1;
            }
        }
    }

    let ans = if let Some(d) = dist[h - 1][w - 1] {
        white_count - d - 1
    } else {
        -1
    };

    println!("{}", ans);
}

fn bfs(
    h: usize,
    w: usize,
    graph: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<Vec<Option<isize>>> {
    let mut dist = vec![vec![None; w]; h];
    let mut q = std::collections::VecDeque::new();
    q.push_back(start);
    dist[start.0][start.1] = Some(0);

    while let Some((i, j)) = q.pop_front() {
        if (i, j) == goal {
            return dist;
        }

        let p = w * i + j;
        for &next in &graph[p] {
            let next_i = next / w;
            let next_j = next % w;
            if dist[next_i][next_j].is_none() {
                dist[next_i][next_j] = Some(dist[i][j].unwrap() + 1);
                q.push_back((next_i, next_j));
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
