use std::{cmp::Ordering, collections::BTreeSet};

use proconio::input;
use std::ops::Bound::Included;

fn main() {
    input! {
        q: usize,
    };

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        match t {
            1 => {
                set.insert(x);
            }
            2 => {
                set.remove(&x);
            }
            3 => {
                if let Some(x) = set.range((Included(&x), Included(&1000000001))).next() {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
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
