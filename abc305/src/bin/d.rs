use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut sum = vec![(0, 0); n];
    for i in 1..n {
        sum[i].0 = sum[i - 1].0 + a[i] - a[i - 1];
        sum[i].1 = sum[i - 1].1;

        if i % 2 == 0 {
            sum[i].1 += a[i] - a[i - 1];
        }
    }

    for (l, r) in lr {
        let r_p = sum.lower_bound(&(r, 0));
        let l_p = sum.lower_bound(&(l, 0));
        let r_s = sum[r_p];
        let l_s = sum[l_p];

        let mut ans = r_s.1 - l_s.1;

        if l_p % 2 == 0 {
            ans += l_s.0 - l;
        }

        if r_p % 2 == 0 {
            ans -= r_s.0 - r;
        }

        println!("{}", ans);
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
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

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
