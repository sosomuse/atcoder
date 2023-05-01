use std::cmp::Ordering;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        _: usize,
        k: usize,
        s: Chars,
    }

    let x_count = s.iter().filter(|&c| *c == 'x').count();
    let mut count = k / x_count;
    let mut res = k % x_count;

    if count != 0 {
        count -= 1;
        res += x_count;
    }

    let ans = n * count;

    let mut t = vec![];
    for _ in 0..5 {
        t.append(&mut s.clone());
    }
    let mut x = vec![0; t.len() + 1];
    for i in 0..t.len() {
        if t[i] == 'x' {
            x[i + 1] = x[i] + 1;
        } else {
            x[i + 1] = x[i];
        }
    }

    let mut max_count = 0;

    for i in 0..x.len() {
        let a = x[i];
        let count = x.lower_bound(&(a + res + 1));
        max_count = std::cmp::max(max_count, count - i - 1);
    }

    println!("{}", ans + max_count);
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
