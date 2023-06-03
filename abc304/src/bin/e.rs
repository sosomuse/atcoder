use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        k: usize,
        bad: [(Usize1, Usize1); k],
        q: usize,
        queries: [(Usize1, Usize1); q],
    }

    let mut uf = RollbackUnionFind::new(n);
    for (u, v) in edges {
        uf.unite(u, v);
    }

    let bad = bad.into_iter().collect::<HashSet<_>>();
    let mut roots = HashSet::new();

    for (x, y) in bad {
        roots.insert((uf.find(x), uf.find(y)));
    }

    for (u, v) in &queries {
        uf.snapshot();
        uf.unite(*u, *v);
        if roots.contains(&(uf.find(*u), uf.find(*v)))
            || roots.contains(&(uf.find(*v), uf.find(*u)))
        {
            println!("No");
        } else {
            println!("Yes");
        }
        uf.undo();
    }
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

pub struct RollbackUnionFind {
    data: Vec<i32>,
    history: VecDeque<(usize, i32)>,
    inner_snap: usize,
}

impl RollbackUnionFind {
    pub fn new(sz: usize) -> Self {
        RollbackUnionFind {
            data: vec![-1; sz],
            history: VecDeque::new(),
            inner_snap: 0,
        }
    }

    pub fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find(x);
        y = self.find(y);
        self.history.push_back((x, self.data[x]));
        self.history.push_back((y, self.data[y]));
        if x == y {
            return false;
        }
        if self.data[x] > self.data[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.data[x] += self.data[y];
        self.data[y] = x as i32;
        true
    }

    pub fn find(&mut self, k: usize) -> usize {
        if self.data[k] < 0 {
            k
        } else {
            self.find(self.data[k] as usize)
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size(&mut self, k: usize) -> i32 {
        let x = self.find(k);
        -self.data[x]
    }

    pub fn undo(&mut self) {
        let history_data = self.history.pop_back().unwrap();
        self.data[history_data.0] = history_data.1;
        let history_data = self.history.pop_back().unwrap();
        self.data[history_data.0] = history_data.1;
    }

    pub fn snapshot(&mut self) {
        self.inner_snap = self.history.len() >> 1;
    }

    pub fn get_state(&self) -> usize {
        self.history.len() >> 1
    }

    pub fn rollback(&mut self, mut state: i32) -> () {
        if state == -1 {
            state = self.inner_snap as i32;
        };

        state <<= 1;
        while state < self.history.len() as i32 {
            self.undo();
        }
    }
}
