use proconio::input;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();

    let mut count = HashMap::<usize, usize>::new();
    let mut ans = 0;

    for i in 0..n {
        count.entry(a[i]).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut y = HashMap::<usize, Vec<usize>>::new();

    for i in 0..a.len() {
        let v = a[i];
        let s = solve(v);
        y.insert(v, s);
    }

    for i in 0..a.len() {
        let v = a[i];
        if let Some(s) = y.get(&v) {
            for j in 0..s.len() {
                let cnt_j = count.get(&s[j]).unwrap_or(&0);
                let cnt_k = count.get(&(a[i] / s[j])).unwrap_or(&0);
                ans += cnt_j * cnt_k;
            }
        }
    }

    println!("{}", ans);
}

fn solve(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i * i != n {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
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
