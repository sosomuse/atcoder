use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut x = vec![0; n + 1];
    let mut y = vec![0; n + 1];

    for i in 0..n {
        x[i + 1] = x[i];
        y[i + 1] = y[i];
        match s[i] {
            'o' => x[i + 1] += 1,
            'x' => y[i + 1] += 1,
            _ => unreachable!(),
        }
    }

    let mut ans = 0;

    for l in 0..n {
        let x_count = x[l];
        let y_count = y[l];

        let x_i = x.lower_bound(&(x_count + 1));
        let y_i = y.lower_bound(&(y_count + 1));

        ans += n - (std::cmp::max(x_i, y_i) - 1);
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
