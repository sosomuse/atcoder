use proconio::{input, marker::Usize1};

// 最小全域木
fn main() {
    input! {
        n: usize,
        m: usize,
        mut edges: [(Usize1, Usize1, isize); m],
    };

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(n);
    let mut ans = 0;

    for (_, _, c) in edges.iter() {
        if c > &0 {
            ans += c;
        }
    }

    for (u, v, c) in edges {
        if !uf.is_same(u, v) {
            uf.unite(u, v);

            if c > 0 {
                ans -= c;
            }
        }
    }

    println!("{}", ans);
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
