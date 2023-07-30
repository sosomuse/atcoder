use proconio::input;
use std::cmp::Ordering;

pub trait BinarySearch<T: Ord> {
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

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();

    let (mut lo, mut hi) = (1, *b.last().unwrap() + 1);

    while lo < hi {
        let mid = (hi + lo) / 2;
        if !check(&a, &b, mid) {
            lo = mid + 1
        } else {
            hi = mid;
        }
    }

    println!("{}", lo);
}

fn check(a: &[i64], b: &[i64], x: i64) -> bool {
    let sellers = a.upper_bound(&x);
    let buyers = b.len() - b.lower_bound(&(x));
    sellers >= buyers
}
