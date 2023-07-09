use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n-1],
        xy: [(Usize1, usize); m],
    }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[p[i]].push(i + 1);
    }

    let mut dist: Vec<usize> = vec![0; g.len()];
    let mut is_target = vec![false; g.len()];

    for (x, y) in xy {
        is_target[x] = true;
        dist[x] = dist[x].max(y);
    }

    dfs(0, &g, &mut dist, &mut is_target);

    let mut ans = 0;
    for i in 0..n {
        if is_target[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, dist: &mut Vec<usize>, is_target: &mut Vec<bool>) {
    for &w in &g[v] {
        if dist[v] >= 1 {
            is_target[w] = true;
            dist[w] = std::cmp::max(dist[w], dist[v] - 1);
        }
        dfs(w, g, dist, is_target);
    }
}
