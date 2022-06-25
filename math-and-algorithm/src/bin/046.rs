use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        (sx, sy): (usize, usize),
        (gx, gy): (usize, usize),
        grid: [Chars; r],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; r * c + 1];

    let is_moving = |c: char| match c {
        '.' => true,
        _ => false,
    };

    let get_pos = |x: usize, y: usize| (x * c + y) as usize;

    for i in 0..r {
        for j in 0..c {
            let g = grid[i][j];

            if is_moving(g) {
                let pos = get_pos(i, j);
                if i > 0 {
                    let up = grid[i - 1][j];
                    if is_moving(up) {
                        graph[pos].push(get_pos(i - 1, j));
                    }
                }

                if j > 0 {
                    let left = grid[i][j - 1];
                    if is_moving(left) {
                        graph[pos].push(get_pos(i, j - 1));
                    }
                }

                let down = grid[i + 1][j];

                if i < r {
                    if is_moving(down) {
                        graph[pos].push(get_pos(i + 1, j));
                    }
                }

                if j < c {
                    let right = grid[i][j + 1];

                    if is_moving(right) {
                        graph[pos].push(get_pos(i, j + 1));
                    }
                }
            }
        }
    }

    let start = get_pos(sx - 1, sy - 1);
    let end = get_pos(gx - 1, gy - 1);
    let ans = bfs(start, &graph);

    println!("{}", ans[end]);
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
