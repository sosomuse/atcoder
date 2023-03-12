use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };

    let mut graph = vec![vec![]; h * w];
    let mut s = vec![];

    for i in 0..h {
        for j in 0..w {
            s.push(a[i][j]);
        }
    }

    for i in 0..h {
        for j in 0..w {
            if i + 1 < h {
                graph[i * w + j].push(i * w + j + w);
            }
            if j + 1 < w {
                graph[i * w + j].push(i * w + j + 1);
            }
        }
    }

    let ans = bfs(0, &graph, s);

    println!("{}", ans);
}

fn bfs(v: usize, graph: &Vec<Vec<usize>>, a: Vec<usize>) -> usize {
    let mut queue: VecDeque<(usize, Vec<usize>)> = std::collections::VecDeque::new();
    let mut ans = 0;
    let mut set = vec![];
    set.push(a[v]);
    queue.push_front((v, set.clone()));

    while !queue.is_empty() {
        let (pos, n_set) = queue.pop_front().unwrap();
        if pos == &graph.len() - 1 {
            ans += 1;
            continue;
        }

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            if n_set.contains(&a[nex]) {
                continue;
            }
            let mut n_set = n_set.clone();
            n_set.push(a[nex]);
            queue.push_back((nex, n_set));
        }
    }

    ans
}
