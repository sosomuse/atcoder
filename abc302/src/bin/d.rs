use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    };

    a.dedup();
    b.dedup();

    a.sort();
    b.sort();

    let mut ans = -1;

    for i in 0..a.len() {
        let x = a[i];
        let y = b.lower_bound(&(x + d));

        for j in -1..=1 {
            let z = y as isize + j;
            if z < 0 || z >= b.len() as isize {
                continue;
            }

            let z = z as usize;
            if (x as isize - b[z] as isize).abs() as usize <= d {
                ans = ans.max((x + b[z]) as isize);
            }
        }
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
