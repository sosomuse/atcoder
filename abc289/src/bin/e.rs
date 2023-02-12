use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
            uv: [(Usize1, Usize1); m],
        };

        let mut graph = vec![vec![]; n];
        for (u, v) in uv {
            graph[u].push(v);
            graph[v].push(u);
        }

        let (dist1, dist2, ok) = bfs(0, n - 1, &graph, &c);

        if !ok {
            println!("-1");
            continue;
        }
        println!("{}", dist1[n - 1].max(dist2[0]));
    }
}

fn bfs(
    v: usize,
    n: usize,
    graph: &Vec<Vec<usize>>,
    colors: &Vec<usize>,
) -> (Vec<isize>, Vec<isize>, bool) {
    let mut taka_dist: Vec<isize> = vec![-1; graph.len()];
    let mut ao_dist: Vec<isize> = vec![-1; graph.len()];
    let mut queue: VecDeque<(usize, usize)> = std::collections::VecDeque::new();
    queue.push_front((v, n));
    taka_dist[v] = 0;
    ao_dist[n] = 0;
    let mut ok = false;

    while !queue.is_empty() {
        let (t, a) = *queue.front().unwrap();

        if t == n && a == 0 {
            ok = true;
            break;
        }

        queue.pop_front().unwrap();

        let ta = &graph[t];
        let ao = &graph[a];

        for i in 0..ta.len() {
            let nex_t = ta[i];

            for j in 0..ao.len() {
                let nex_ao = ao[j];

                if colors[nex_t] != colors[nex_ao] {
                    taka_dist[nex_t] = taka_dist[t] + 1;
                    ao_dist[nex_ao] = ao_dist[a] + 1;
                    queue.push_back((nex_t, nex_ao));
                }
            }
        }
    }

    (taka_dist, ao_dist, ok)
}
