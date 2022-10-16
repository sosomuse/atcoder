use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    };

    a.sort();

    for _ in 0..q {
        input! {
            x: usize,
        };

        println!("{}", a.lower_bound(&x).unwrap_or(a.len()));
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> Option<usize>;
    fn upper_bound(&self, x: &T) -> Option<usize>;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> Option<usize> {
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

        if self.len() > low && self[low] >= *x {
            Some(low)
        } else {
            None
        }
    }

    fn upper_bound(&self, x: &T) -> Option<usize> {
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
        if self.len() > low && self[low] > *x {
            Some(low)
        } else {
            None
        }
    }
}
