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

    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(uf.root(i));
    }

    for v in set {
        let (vertices, edges) = bfs(v, &graph);

        if vertices != edges {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn bfs(v: usize, graph: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut dist = vec![-1; graph.len()];
    let mut vertices = 0;
    let mut edges = 0;
    let mut queue = std::collections::VecDeque::<usize>::new();
    queue.push_front(v);
    dist[v] = 0;

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();
        queue.pop_front().unwrap();
        vertices += 1;

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            edges += 1;
            if dist[nex] == -1 {
                dist[nex] = dist[pos] + 1;
                queue.push_back(nex);
            }
        }
    }

    (vertices, edges / 2)
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

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return;
        }

        if self.siz[x] < self.siz[y] {
            self.par[x] = y;
            self.siz[y] += self.siz[x];
        } else {
            self.par[y] = x;
            self.siz[x] += self.siz[y];
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.siz[x]
    }
}
