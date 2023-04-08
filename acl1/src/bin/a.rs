use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let mut xyi: Vec<(usize, usize, usize)> = xy
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (x, y, i))
        .collect();

    xyi.sort();
    let mut uf = UnionFind::new(n);
    let mut btree = BTreeMap::new();

    for &(_, y, i) in xyi.iter() {
        if btree.range(..y).count() == 0 {
            btree.insert(y, i);
            continue;
        }

        let mut stack = vec![];

        for (y2, j) in btree.range(..y) {
            stack.push(*y2);
            uf.unite(i, *j);
        }

        while stack.len() > 1 {
            btree.remove(&stack.pop().unwrap());
        }
    }
    for i in 0..n {
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
