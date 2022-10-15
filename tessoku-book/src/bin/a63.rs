use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let ans = bfs(&graph, 1);

    for v in 1..ans.len() {
        println!("{} ", ans[v]);
    }
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
