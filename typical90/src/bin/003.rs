use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in e {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dist = vec![n; n];
    dist[0] = 0;
    let mut m = 0;
    let mut mi = 0;
    dfs(0, &g, &mut dist, &mut m, &mut mi);
    let mut dist = vec![n; n];
    dist[mi] = 0;
    let mut m = 0;
    dfs(mi, &g, &mut dist, &mut m, &mut mi);
    println!("{}", m + 1)
}

fn dfs(v: usize, g: &[Vec<usize>], dist: &mut [usize], m: &mut usize, mi: &mut usize) {
    for &nv in g[v].iter() {
        if dist[nv] != dist.len() {
            continue;
        }
        dist[nv] = dist[v] + 1;
        if *m < dist[nv] {
            *m = dist[nv];
            *mi = nv;
        }
        dfs(nv, g, dist, m, mi);
    }
}
