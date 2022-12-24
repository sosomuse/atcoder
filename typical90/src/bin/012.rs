use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(h * w);
    let mut set = BTreeSet::new();

    let get_grid = |r: usize, c: usize| -> usize { (r - 1) * w + c - 1 };

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                r: usize,
                c: usize,
            }

            let rx = get_grid(r, c);
            set.insert(rx);

            if r > 1 {
                let up = get_grid(r - 1, c);
                if set.contains(&up) {
                    uf.unite(rx, up);
                }
            }

            if r < h {
                let down = get_grid(r + 1, c);
                if set.contains(&down) {
                    uf.unite(rx, down);
                }
            }

            if c > 1 {
                let left = get_grid(r, c - 1);
                if set.contains(&left) {
                    uf.unite(rx, left);
                }
            }

            if c < w {
                let right = get_grid(r, c + 1);
                if set.contains(&right) {
                    uf.unite(rx, right);
                }
            }
        } else if t == 2 {
            input! {
                r1: usize,
                c1: usize,
                r2: usize,
                c2: usize,
            }

            let rx1 = get_grid(r1, c1);
            let rx2 = get_grid(r2, c2);

            if set.contains(&rx1) && set.contains(&rx2) && uf.issame(rx1, rx2) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
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
