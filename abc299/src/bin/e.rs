use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
        k: usize,
        pd: [(usize, usize); k],
    };

    let mut graph = vec![vec![]; n + 1];
    for i in 0..m {
        let (u, v) = uv[i];
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dists = vec![vec![]; n + 1];
    for i in 1..=n {
        dists[i] = bfs(i, &graph);
    }

    let mut ans = vec![1; n + 1];
    for i in 0..k {
        let (p, d) = pd[i];
        let mut is_ok = false;

        for j in 1..=n {
            if dists[p][j] == d as isize {
                if ans[j] != 0 {
                    is_ok = true;
                }
            }

            if dists[p][j] >= 0 && dists[p][j] < d as isize {
                ans[j] = 0;
            }
        }

        if !is_ok {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for i in 1..=n {
        print!("{}", ans[i]);
    }
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
