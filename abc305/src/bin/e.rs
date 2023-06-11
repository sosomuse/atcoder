use proconio::input;
use proconio::marker::Usize1;
use std::collections::{BTreeSet, VecDeque};

struct Guard {
    pos: usize,
    power: isize,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1); m],
        guards: [(Usize1, isize); k],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut guards: Vec<_> = guards
        .into_iter()
        .map(|(pos, power)| Guard { pos, power })
        .collect();

    guards.sort_by(|a, b| b.power.cmp(&a.power));

    let mut secure = vec![false; n];
    let mut powers = vec![-1; n];

    for guard in guards {
        let Guard { pos, power } = guard;

        if power > powers[pos] {
            powers[pos] = power;
            secure[pos] = true;

            let mut queue = VecDeque::new();
            queue.push_back((pos, power));

            while let Some((v, p)) = queue.pop_front() {
                if p == 0 {
                    continue;
                }

                for &w in &graph[v] {
                    if p - 1 > powers[w] {
                        secure[w] = true;
                        powers[w] = p - 1;
                        queue.push_back((w, p - 1));
                    }
                }
            }
        }
    }

    let secure_nodes = secure
        .into_iter()
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(i, _)| i + 1)
        .collect::<BTreeSet<_>>();

    println!("{}", secure_nodes.len());
    for node in secure_nodes {
        println!("{}", node);
    }
}
