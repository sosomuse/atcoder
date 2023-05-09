use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let graph = create_maze_graph(h, w, &c, '#');
    let mut ans = -1;

    for i in 0..h {
        for j in 0..w {
            let p = w * i + j;
            let tmp = dfs(p, &graph);
            ans = std::cmp::max(ans, tmp);
        }
    }

    println!("{}", ans);
}

fn dfs(start: usize, graph: &Vec<Vec<usize>>) -> isize {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut max = -1;
    _dfs(start, graph, &mut visited, 0, start, &mut max);
    max
}

fn _dfs(
    v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    count: usize,
    goal: usize,
    max: &mut isize,
) {
    for &w in graph[v].iter() {
        if w == goal {
            if count >= 3 {
                *max = std::cmp::max(*max, count as isize + 1);
            }
            continue;
        }

        if !visited[w] {
            visited[w] = true;
            _dfs(w, graph, visited, count + 1, goal, max);
            visited[w] = false;
        }
    }
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
