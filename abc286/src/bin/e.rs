use proconio::{input, marker::Chars};
use std::cmp::min;

const INF: i64 = 1_000_000_000_000_000_000;

// ワーシャルフロイド法
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        s: [Chars; n],
        q: usize,
        uv: [(usize, usize); q],
    }

    let mut graph = vec![vec![(INF, 0); n]; n];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                graph[i][j] = (1, -a[j]);
            }
        }
    }

    let add = |a: (i64, i64), b: (i64, i64)| (a.0 + b.0, a.1 + b.1);

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] = min(graph[i][j], add(graph[i][k], graph[k][j]));
            }
        }
    }

    for (u, v) in uv {
        let (d, c) = graph[u - 1][v - 1];
        if d == INF {
            println!("Impossible");
        } else {
            println!("{} {}", d, -c + a[u - 1]);
        }
    }
}
