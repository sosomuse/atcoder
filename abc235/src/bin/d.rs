use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    };

    let limit = 1000000;
    let mut queue = VecDeque::new();
    let mut graph = vec![vec![]; limit];

    queue.push_back(1);

    loop {
        if queue.is_empty() {
            break;
        }

        let b = queue.pop_front().unwrap();

        if b >= limit {
            continue;
        }

        if b * a < limit && !graph[b].contains(&(b * a)) {
            graph[b].push(b * a);
            queue.push_back(b * a);
        }

        if b >= 10 && b % 10 != 0 {
            let c = rotate_last(b);

            if graph[c].contains(&b) || graph[b].contains(&c) {
                continue;
            }

            graph[b].push(c);
            queue.push_back(c);
        }
    }

    let ans = bfs(1, &graph);

    println!("{}", ans[n]);
}

// 最後の桁を1桁目にする
fn rotate_last(n: usize) -> usize {
    let len = n.to_string().len();
    let p = 10usize.pow(len as u32 - 1);
    let last = n % 10;
    let x = n / 10;

    x + last * p
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
