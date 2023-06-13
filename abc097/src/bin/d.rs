use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
        xy: [(usize, usize); m],
    };

    let mut indexes = vec![0; n + 1];

    for (i, &pi) in p.iter().enumerate() {
        indexes[pi] = i + 1;
    }

    let mut uf = UnionFind::new(n + 1);
    for (x, y) in xy {
        let xi = indexes[x];
        let yi = indexes[y];
        uf.unite(xi, yi);
    }

    let mut ans = 0;
    for i in 1..=n {
        if uf.is_same(i, indexes[i]) {
            ans += 1;
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
