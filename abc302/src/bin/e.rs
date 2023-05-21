use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut graph = vec![BTreeSet::new(); n];
    let mut unconnected_count = n;

    for _ in 0..q {
        input! {
            t: usize,
            u: Usize1,
        };

        if t == 1 {
            input! {
                v: Usize1,
            };
            if graph[u].len() == 0 {
                unconnected_count -= 1;
            }
            if graph[v].len() == 0 {
                unconnected_count -= 1;
            }

            graph[u].insert(v);
            graph[v].insert(u);
        } else {
            let vec = graph[u].iter().cloned().collect::<Vec<usize>>();

            for v in vec {
                graph[v].remove(&u);
                if graph[v].len() == 0 {
                    unconnected_count += 1;
                }
            }

            if graph[u].len() != 0 {
                graph[u].clear();
                unconnected_count += 1;
            }
        }

        println!("{}", unconnected_count);
    }
}
