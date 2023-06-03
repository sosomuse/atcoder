use std::{cmp::Ordering, collections::HashMap};

use proconio::input;

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        strawberry: [(usize, usize); n],
        a: usize,
        x: [usize; a],
        b: usize,
        y: [usize; b],
    }

    let mut table = HashMap::new();
    for (px, py) in strawberry {
        let x_idx = x.lower_bound(&px);
        let y_idx = y.lower_bound(&py);
        *table.entry((x_idx, y_idx)).or_insert(0) += 1;
    }

    let max = *table.values().max().unwrap_or(&0);
    let mut min = *table.values().min().unwrap_or(&0);

    if table.len() < (a + 1) * (b + 1) {
        min = 0;
    }

    println!("{} {}", min, max);
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
