use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut a = ab.iter().map(|&(a, _)| a).collect::<Vec<_>>();
    let mut b = ab.iter().map(|&(_, b)| b).collect::<Vec<_>>();

    a.sort();
    b.sort();

    a.dedup();
    b.dedup();

    for &(av, bv) in &ab {
        let ap = a.lower_bound(&av) + 1;
        let bp = b.lower_bound(&bv) + 1;

        println!("{} {}", ap, bp);
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
