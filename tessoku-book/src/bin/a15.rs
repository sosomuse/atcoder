use std::cmp::Ordering;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut b = a.clone();
    b.sort();

    let c: Vec<_> = b.into_iter().unique().collect();

    let mut ans = vec![0; n];

    for i in 0..n {
        let v = a[i];
        let x = c.lower_bound(&v);
        ans[i] = x + 1;
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
