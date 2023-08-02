use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tx: [(usize, usize); n],
    }

    let mut cans = vec![];
    let mut need_opener_cans = vec![];
    let mut openers = vec![];

    for (t, x) in tx {
        match t {
            0 => cans.push(x),
            1 => need_opener_cans.push(x),
            2 => openers.push(x),
            _ => unreachable!(),
        }
    }

    cans.sort_by(|a, b| b.cmp(a));
    need_opener_cans.sort_by(|a, b| b.cmp(a));
    openers.sort_by(|a, b| b.cmp(a));

    let mut cans_s = vec![0; cans.len() + 1];
    let mut need_opener_cans_s = vec![0; need_opener_cans.len() + 1];
    let mut s = vec![0; openers.len() + 1];
    for i in 0..cans.len() {
        cans_s[i + 1] = cans_s[i] + cans[i];
    }
    for i in 0..need_opener_cans.len() {
        need_opener_cans_s[i + 1] = need_opener_cans_s[i] + need_opener_cans[i];
    }
    for i in 0..openers.len() {
        s[i + 1] = s[i] + openers[i];
    }

    let mut ans = 0;

    for i in 0..m {
        let cans_count = i;
        let mut need_opener_cans_count = m - i;

        if cans_s.len() <= cans_count {
            continue;
        }

        let mut left = 0;
        let mut right = need_opener_cans_count;

        while left < right {
            let mid = (left + right) / 2;
            let s_count = need_opener_cans_count - mid;
            if s[s_count.min(s.len() - 1)] >= mid {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        need_opener_cans_count = left - 1;

        let cans_sum = cans_s[cans_count];
        let need_opener_cans_sum = need_opener_cans_s[need_opener_cans_count];

        ans = ans.max(cans_sum + need_opener_cans_sum);
    }

    println!("{}", ans);
}

pub trait BinarySearch<T> {
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
