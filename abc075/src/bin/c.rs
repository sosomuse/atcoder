use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut ans = 0;

    for i in 0..m {
        let mut g = vec![vec![]; n];
        for j in 0..m {
            if i == j {
                continue;
            }

            let (a, b) = ab[j];
            g[a].push(b);
            g[b].push(a);
        }

        let mut visited = vec![false; n];
        dfs(0, &g, &mut visited);

        if visited.iter().any(|&v| !v) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[v] = true;

    for &w in &g[v] {
        if visited[w] {
            continue;
        }
        dfs(w, g, visited);
    }
}
