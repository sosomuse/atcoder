use std::collections::{BTreeSet, VecDeque};

use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: usize,
        t: usize,
        x: usize,
        uv: [(usize, usize); m],
    };

    let mut edge = vec![vec![]; n + 1];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }

    // n手目に到達できる頂点の集合
    let mut vec = vec![BTreeSet::<usize>::new(); k + 1];
    let mut queue = VecDeque::new();

    vec[k].insert(t);
    queue.push_back((k, t));

    while let Some((dist, position)) = queue.pop_front() {
        if dist == 0 {
            continue;
        }

        for &next in edge[position].iter() {
            if dist == 1 && next != s {
                continue;
            }

            if vec[dist - 1].contains(&next) {
                continue;
            }
            vec[dist - 1].insert(next);
            queue.push_back((dist - 1, next));
        }
    }

    // dp [i][j][k] := i手目に頂点jにいて、k=0ならxを偶数回通過している、k=1なら奇数回通過している
    let mut dp = vec![vec![vec![0usize; 2]; n + 1]; k + 1];
    if s == x {
        dp[0][s][1] = 1;
    } else {
        dp[0][s][0] = 1;
    }

    for i in 0..k {
        for &current in vec[i].iter() {
            let nexts: Vec<usize> = edge[current]
                .iter()
                .filter(|&&v| vec[i + 1].contains(&v))
                .map(|&v| v)
                .collect();

            for &next in nexts.iter() {
                if next == x {
                    dp[i + 1][next][1] += dp[i][current][0];
                    dp[i + 1][next][0] += dp[i][current][1];
                } else {
                    dp[i + 1][next][1] += dp[i][current][1];
                    dp[i + 1][next][0] += dp[i][current][0];
                }

                dp[i + 1][next][1] %= MOD;
                dp[i + 1][next][0] %= MOD;
            }
        }
    }

    println!("{}", dp[k][t][0]);
}
