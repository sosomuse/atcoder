use std::vec;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    let mut seg_tree = SegmentTree::from(&a, 0);

    for _ in 0..q {
        input! {
            t: usize,
            x: Usize1,
            y: usize,
        };

        if t == 1 {
            let v = seg_tree.get_point(x);
            seg_tree.update(x, v ^ y);
        } else {
            println!("{}", seg_tree.get(x, y));
        }
    }
}

struct SegmentTree {
    size: usize,
    tree: Vec<usize>,
    element: usize,
}

impl SegmentTree {
    fn new(n: usize, element: usize) -> Self {
        let size = n.next_power_of_two();

        Self {
            size,
            tree: vec![element; size << 1],
            element,
        }
    }

    fn update(&mut self, pos: usize, x: usize) {
        let mut pos = pos + self.size;
        self.tree[pos] = x;
        while pos > 1 {
            pos >>= 1;
            let lch = pos << 1;
            self.tree[pos] = self.tree[lch] ^ (self.tree[lch + 1]);
        }
    }

    fn from(arr: &Vec<usize>, element: usize) -> Self {
        let mut seg = Self::new(arr.len(), element);
        for (i, &v) in arr.iter().enumerate() {
            seg.tree[seg.size + i] = v;
        }
        for i in (0..seg.size).rev() {
            let lch = i << 1;
            seg.tree[i] = seg.tree[lch] ^ seg.tree[lch + 1];
        }
        seg
    }

    fn get_point(&self, index: usize) -> usize {
        let i = self.size + index;
        self.tree[i]
    }

    fn get(&self, left: usize, right: usize) -> usize {
        self._get(left, right)
    }

    fn _get(&self, left: usize, right: usize) -> usize {
        let mut l = self.size + left;
        let mut r = self.size + right;
        let (mut res_l, mut res_r) = (self.element, self.element);

        while l < r {
            if l & 1 == 1 {
                res_l = res_l ^ self.tree[l];
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = self.tree[r] ^ res_r;
            }
            l >>= 1;
            r >>= 1;
        }

        return res_l ^ res_r;
    }
}
