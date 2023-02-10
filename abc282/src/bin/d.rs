use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];
    let mut uf = UnionFind::new(n);

    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
        uf.unite(u, v);
    }

    let mut edges = HashSet::new();

    for i in 0..n {
        edges.insert(uf.root(i));
    }

    let mut colors = vec![0; n];
    let mut ans = n * (n - 1) / 2 - m;

    for i in 0..n {
        if colors[i] != 0 {
            continue;
        }

        let mut cvs = vec![0; 2];
        let bipartite = is_bipartite(i, &graph, &mut colors, &mut cvs, 1);

        if !bipartite {
            println!("0");
            return;
        }

        for s in cvs {
            ans -= (s * (s - 1) / 2) as usize
        }
    }

    println!("{}", ans);
}

// 2部グラフかどうかを判定する
fn is_bipartite(
    v: usize,
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<isize>,
    cvs: &mut Vec<isize>,
    color: isize,
) -> bool {
    colors[v] = color;

    if color == 1 {
        cvs[1] += 1;
    } else {
        cvs[0] += 1;
    }

    for &next in &graph[v] {
        if colors[next] == color {
            return false;
        }

        if colors[next] == 0 && !is_bipartite(next, graph, colors, cvs, -color) {
            return false;
        }
    }

    true
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    #[allow(dead_code)]
    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
