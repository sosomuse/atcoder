use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        uv: [(usize, usize); q],
    };

    let mut graph = vec![vec![]; n + 1];

    for i in 0..n {
        let chars = &s[i];

        for j in 0..n {
            if chars[j] == 'Y' {
                graph[i + 1].push(j + 1);
            }
        }
    }

    for (u, v) in uv {
        let dist = bfs(u, &graph);

        if dist[v] == -1 {
            println!("Impossible");
            continue;
        }

        let ans = dist[v];
        let worth = bfs_with_worth(v, dist, &a);

        println!("{} {}", ans, worth);
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

fn bfs_with_worth(v: usize, dist: Vec<isize>, a: &Vec<usize>) -> usize {
    let mut queue: VecDeque<(usize, usize)> = std::collections::VecDeque::new();
    queue.push_front((v, a[v - 1]));
    let mut routes = vec![vec![]; dist[v] as usize];
    let mut ans = 0;

    for i in 0..routes.len() {
        for j in 0..dist.len() {
            if dist[j] as usize == i {
                routes[i].push(j)
            }
        }
    }

    while !queue.is_empty() {
        let (pos, worth) = queue.pop_front().unwrap();

        if dist[pos] == 0 {
            ans = ans.max(worth);
            continue;
        }

        let t = dist[pos] as usize - 1;

        for i in 0..routes[t].len() {
            let nex = routes[t][i];

            queue.push_back((nex, a[nex - 1] + worth));
        }
    }

    ans
}
