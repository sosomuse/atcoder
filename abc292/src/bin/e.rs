use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
    }

    let mut ans = 0;

    for i in 0..n {
        let mut count = 0;
        bfs(i, &mut graph, &mut count);

        ans += count;
    }

    println!("{}", ans);
}

fn bfs(v: usize, graph: &mut Vec<Vec<usize>>, count: &mut usize) -> Vec<isize> {
    let mut dist = vec![-1; graph.len()];
    let mut queue = std::collections::VecDeque::<usize>::new();
    queue.push_front(v);
    dist[v] = 0;

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();
        queue.pop_front().unwrap();

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            if dist[nex] == -1 {
                if dist[pos] == 1 {
                    *count += 1;
                    dist[nex] = 1;
                } else {
                    dist[nex] = dist[pos] + 1;
                }

                queue.push_back(nex);
            }
        }
    }

    dist
}
