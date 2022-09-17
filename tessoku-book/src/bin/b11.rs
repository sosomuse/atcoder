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

        println!("{}", a.lower_bound(&x));
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
