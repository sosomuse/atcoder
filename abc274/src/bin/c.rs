use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut graph = vec![vec![]; 2 * n + 2];

    for i in 1..=n {
        let v = a[i - 1];
        graph[v].push(i * 2);
        graph[v].push(i * 2 + 1);
    }

    let ans = bfs(&graph, 1);

    for i in 1..ans.len() {
        println!("{}", ans[i]);
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
