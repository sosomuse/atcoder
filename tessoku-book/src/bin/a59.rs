use std::vec;

use proconio::input;

// セグメント木 - l, r合計値
struct SegmentTree {
    size: usize,
    tree: Vec<usize>,
    element: usize,
}

impl SegmentTree {
    fn new(n: usize, element: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            size,
            tree: vec![element; size * 2],
            element,
        }
    }

    pub fn update(&mut self, index: usize, x: usize) {
        let mut i = self.size + index - 1;
        self.tree[i] = x;
        while i >= 2 {
            i /= 2;
            self.tree[i] = self.tree[i * 2] + self.tree[i * 2 + 1];
        }
    }

    fn get(&self, left: usize, right: usize) -> usize {
        return self._get(left, right, 1, self.size + 1, 1);
    }

    fn _get(&self, left: usize, right: usize, a: usize, b: usize, u: usize) -> usize {
        if right <= a || b <= left {
            return self.element;
        }
        if left <= a && b <= right {
            return self.tree[u];
        }
        let m = (a + b) / 2;
        let a_l = self._get(left, right, a, m, u * 2);
        let a_r = self._get(left, right, m, b, u * 2 + 1);
        return a_l + a_r;
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut seg_tree = SegmentTree::new(n, 0);

    for _ in 0..q {
        input! {
            t: usize,
        };

        if t == 1 {
            input! {
                pos: usize,
                x: usize,
            }

            seg_tree.update(pos, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }

            println!("{}", seg_tree.get(l, r));
        }
    }
}
