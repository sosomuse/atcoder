use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        t: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
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

            if t > 0 && c[i][j - 1] == '.' {
                graph[t * i + j].push(t * i + j - 1);
            }

            if j != (t - 1) && c[i][j + 1] == '.' {
                graph[t * i + j].push(t * i + j + 1);
            }
        }
    }

    let ans = bfs(&graph, (sy - 1) * t + (sx - 1));

    println!("{}", ans[(gy - 1) * t + gx - 1]);
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<isize> {
    let mut ans: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    ans[start] = 0;
    queue.push_front(start);

    while let Some(prev) = queue.pop_front() {
        for v in graph[prev].iter() {
            if ans[*v] == -1 {
                ans[*v] = ans[prev] + 1;
                queue.push_back(*v);
            }
        }
    }

    ans
}
