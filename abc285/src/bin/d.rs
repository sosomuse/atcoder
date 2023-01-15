use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut uf = UnionFind::new(n * 2 + 1);
    let mut map = HashMap::new();

    for (i, (s, t)) in st.iter().enumerate() {
        if !map.contains_key(s) {
            map.entry(s).or_insert(i);
        }

        if !map.contains_key(t) {
            map.entry(t).or_insert(n + i);
        }
    }

    for (a, b) in st.iter() {
        let a = *map.get(a).unwrap();
        let b = *map.get(b).unwrap();

        // 閉路検出
        if uf.issame(a, b) {
            println!("No");
            return;
        }

        uf.unite(a, b);
    }

    println!("Yes");
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
