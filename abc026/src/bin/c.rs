use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    };

    let mut g = vec![vec![]; n];
    for (i, &v) in a.iter().enumerate() {
        g[v].push(i + 1);
    }

    let mut values = vec![0; n];
    dfs(0, &g, &mut values);

    println!("{}", values[0]);
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, values: &mut Vec<usize>) {
    let mut s = BTreeSet::new();

    for &next in g[v].iter() {
        if values[next] == 0 {
            dfs(next, g, values);
        }

        s.insert(values[next]);
    }

    if s.len() == 0 {
        values[v] = 1;
    } else {
        let max = *s.iter().max().unwrap();
        let min = *s.iter().min().unwrap();

        values[v] = max + min + 1;
    }
}
