use proconio::input;

// 最大全域木
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut edges = vec![];
    for i in 0..m {
        edges.push((i, abc[i].2));
    }
    edges.sort_by(|a, b| b.1.cmp(&a.1));

    let mut ans = 0;
    let mut uf = UnionFind::new(n + 1);

    for i in 0..edges.len() {
        let idx = edges[i].0;
        let (a, b, c) = abc[idx];
        if !uf.issame(a, b) {
            uf.unite(a, b);
            ans += c;
        }
    }

    println!("{}", ans);
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

// ユニオンファインド
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
