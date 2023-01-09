use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    };

    a.sort();

    let mut uf = UnionFind::new(n);

    for i in 0..n {
        let j = (i + 1) % n;
        if a[i] == a[j] || (a[i] + 1) % m == a[j] {
            uf.unite(i, j);
        }
    }

    let mut s = vec![0; n];
    for i in 0..n {
        s[uf.root(i)] += a[i];
    }

    let sum = a.iter().sum::<usize>();
    let max = s.iter().max().unwrap();

    println!("{}", sum - max);
}

#[derive(Debug)]
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

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
