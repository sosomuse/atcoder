use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut uf = UnionFind::new(n);
    let mut inconvenience = vec![0; m];
    inconvenience[m - 1] = n * (n - 1) / 2;

    for i in (0..m - 1).rev() {
        let (a, b) = ab[i + 1];
        if uf.is_same(a, b) {
            inconvenience[i] = inconvenience[i + 1];
        } else {
            inconvenience[i] = inconvenience[i + 1] - uf.size(a) * uf.size(b);
            uf.unite(a, b);
        }
    }

    for i in 0..m {
        println!("{}", inconvenience[i]);
    }
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
