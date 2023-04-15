use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    };

    a.sort();
    b.sort();
    c.sort();

    let mut s = vec![0; n + 1];

    for i in 0..n {
        let v = b[i];
        let count = a.lower_bound(&v);
        s[i + 1] = s[i] + count;
    }

    let mut ans = 0;

    for i in 0..n {
        let v = c[i];
        let count = b.lower_bound(&v);
        ans += s[count];
    }

    println!("{}", ans);
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
