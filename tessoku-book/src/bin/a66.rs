use proconio::input;

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

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n + 1);

    for _ in 0..q {
        input! {
            i: usize,
            u: usize,
            v: usize,
        }

        if i == 1 {
            uf.unite(u, v);
        }

        if i == 2 {
            if uf.issame(u, v) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
