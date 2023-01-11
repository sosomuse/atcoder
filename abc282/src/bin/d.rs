use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    };

    let mut graph = vec![vec![]; n];
    let mut uf = UnionFind::new(n);

    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
        uf.unite(u - 1, v - 1);
    }

    let mut edges = HashSet::new();

    for i in 0..n {
        edges.insert(uf.root(i));
    }

    let mut colors = vec![0; n];

    for v in edges {
        let bipartite = is_bipartite(v, &graph, &mut colors, 1);
        if !bipartite {
            println!("0");
            return;
        }
    }

    let origin_black = colors.iter().filter(|&&c| c == 1).count();
    let origin_white = colors.iter().filter(|&&c| c == -1).count();

    let mut ans = 0;

    for i in 0..graph.len() {
        let is_black = colors[i] == 1;
        let mut count = {
            if is_black {
                origin_white
            } else {
                origin_black
            }
        };

        for &next in &graph[i] {
            if uf.issame(i, next) {
                count -= 1;
            }
        }

        ans += count;
    }

    println!("{}", ans / 2);
}

// 2部グラフかどうかを判定する
fn is_bipartite(v: usize, graph: &Vec<Vec<usize>>, colors: &mut Vec<isize>, color: isize) -> bool {
    colors[v] = color;

    for &next in &graph[v] {
        if colors[next] == color {
            return false;
        }

        if colors[next] == 0 && !is_bipartite(next, graph, colors, -color) {
            return false;
        }
    }

    true
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

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
