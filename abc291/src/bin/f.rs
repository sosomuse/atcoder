use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut graph = vec![vec![]; n + 1];
    let mut r_graph = vec![vec![]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if s[i - 1][j - 1] == '1' {
                graph[i].push(i + j);
            }
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            if s[i - 1][j - 1] == '1' {
                r_graph[i + j].push(i);
            }
        }
    }

    let dist = bfs(1, &graph);

    if dist[n] == -1 {
        for _ in 2..=n - 1 {
            println!("-1");
        }
        return;
    }

    // 経路復元
    let mut set = std::collections::HashSet::new();
    let mut pos = n;
    while pos != 1 {
        set.insert(pos);
        for i in 0..r_graph[pos].len() {
            let nex = r_graph[pos][i];
            if dist[nex] == dist[pos] - 1 {
                pos = nex;
                break;
            }
        }
    }

    for i in 2..=n - 1 {
        // 最短経路に含まれていない場合、そのまま出力
        if !set.contains(&i) {
            let ans = &dist[n];
            println!("{}", ans);
            continue;
        }

        // 最短経路に含まれている場合、i ± m の範囲で再計算
        let start = if i > m { i - m } else { 1 };
        let end = if i + m > n { n } else { i + m };

        let mut dist2 = vec![-1; end - start + 10];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut priority_queue = std::collections::BinaryHeap::new();

        for j in start..i {
            dist2[j] = dist[j];
            priority_queue.push((-dist[j], j));
        }
        dist2[i] = -1;

        while !priority_queue.is_empty() {
            let (_, pos) = priority_queue.pop().unwrap();
            queue.push_back(pos);
        }

        while !queue.is_empty() {
            let pos = *queue.front().unwrap();
            queue.pop_front().unwrap();

            if pos == end {
                break;
            }

            for k in 0..graph[pos].len() {
                if graph[pos][k] == i {
                    continue;
                }

                let nex = graph[pos][k];
                if dist2[nex] == -1 {
                    dist2[nex] = dist[pos] + 1;
                    queue.push_back(nex);
                }
            }
        }

        let mut ans = -1;

        for j in i + 1..=end - start + 1 {
            if dist2[j] != -1 {
                if ans == -1 {
                    ans = dist2[j] - dist[j];
                } else if ans > dist2[j] - dist[j] {
                    ans = dist2[j] - dist[j];
                }
            }
        }

        dbg!(i, ans, &dist, dist2);

        if ans == -1 {
            println!("-1");
            continue;
        }

        println!("{}", dist[n] + ans);
    }
}

fn bfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<isize> {
    let mut dist: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
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
