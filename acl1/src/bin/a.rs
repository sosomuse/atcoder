use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let mut sorted = xy.clone();
    sorted.sort_by_key(|&(x, _)| x);

    let mut indexes = HashMap::new();
    for (i, (x, y)) in xy.iter().enumerate() {
        indexes.insert((*x, *y), i);
    }

    let mut uf = UnionFind::new(n + 1);

    for i in 1..=n {
        println!("{}", uf.size(i));
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
