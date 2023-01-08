use std::collections::{HashSet, VecDeque};

use proconio::input;

const MAX: usize = 1000000;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];

    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 1;

    bfs(1, &graph, &mut ans);

    println!("{}", ans);
}

struct Node {
    v: usize,
    visited: HashSet<usize>,
}

fn bfs(v: usize, graph: &Vec<Vec<usize>>, ans: &mut usize) -> () {
    let mut queue: VecDeque<Node> = std::collections::VecDeque::new();
    let mut set = HashSet::new();
    set.insert(v);

    queue.push_front(Node { v, visited: set });

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();

        for i in 0..graph[pos.v].len() {
            if *ans >= MAX {
                return;
            }
            let nex = graph[pos.v][i];
            if pos.visited.contains(&nex) {
                continue;
            }
            *ans += 1;
            let mut set = pos.visited.clone();
            set.insert(nex);
            queue.push_back(Node {
                v: nex,
                visited: set,
            });
        }
    }
}

// fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: HashSet<usize>, ans: &mut usize) {
//     if *ans >= MAX {
//         return;
//     }

//     for &w in graph[v].iter() {
//         if !visited.contains(&w) {
//             let mut set = visited.clone();
//             set.insert(w);
//             *ans += 1;
//             dfs(w, graph, set, ans);
//         }
//     }
// }
