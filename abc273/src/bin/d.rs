use std::cmp::Ordering;
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut r: usize,
        mut c: usize,
        n: usize,
        rc: [(usize, usize); n],
        q: usize,
    };

    let mut h_block = HashMap::new();
    let mut w_block = HashMap::new();

    for (r, c) in &rc {
        h_block.entry(*r - 1).or_insert(vec![]).push(*c);
        w_block.entry(*c - 1).or_insert(vec![]).push(*r);
    }

    for v in h_block.values_mut() {
        v.sort();
    }

    for v in w_block.values_mut() {
        v.sort();
    }

    let get_limit = |d: char, l: usize, c: usize, r: usize| {
        let limit = match d {
            'U' => {
                if r > l {
                    r - l
                } else {
                    1
                }
            }
            'D' => h.min(r + l),
            'L' => {
                if c > l {
                    c - l
                } else {
                    1
                }
            }
            'R' => w.min(c + l),
            _ => todo!(),
        };

        limit
    };

    for _ in 0..q {
        input! {
            d: char,
            l: usize,
        }

        match d {
            'L' => {
                if let Some(y) = h_block.get(&(r - 1)) {
                    let mut index = 0;
                    if let Some(t) = y.lower_bound(&(r)) {
                        index = t.max(1);
                    } else {
                        index = y.len();
                    }
                    let z = &y[index - 1];

                    if z >= &c {
                        c = get_limit(d, l, c, r);
                    } else {
                        if z + l >= c {
                            c = z + 1;
                        } else {
                            c -= l
                        }
                    }
                } else {
                    c = get_limit(d, l, c, r);
                }
            }
            'R' => {
                if let Some(y) = h_block.get(&(r - 1)) {
                    if let Some(t) = y.lower_bound(&(c)) {
                        let z = &y[t];
                        if c + l >= *z {
                            c = z - 1;
                        } else {
                            c += l;
                        }
                    } else {
                        c = get_limit(d, l, c, r);
                    }
                } else {
                    c = get_limit(d, l, c, r);
                }
            }
            'U' => {
                if let Some(y) = w_block.get(&(c - 1)) {
                    let mut index = 0;
                    if let Some(t) = y.lower_bound(&(r)) {
                        index = t.max(1);
                    } else {
                        index = y.len();
                    }

                    let z = &y[index - 1];

                    if z >= &r {
                        r = get_limit(d, l, c, r);
                    } else {
                        if z + l >= r {
                            r = z + 1;
                        } else {
                            r -= l
                        }
                    }
                } else {
                    r = get_limit(d, l, c, r);
                }
            }
            'D' => {
                if let Some(y) = w_block.get(&(c - 1)) {
                    if let Some(t) = y.lower_bound(&(r)) {
                        let z = &y[t];

                        if r + l >= *z {
                            r = z - 1;
                        } else {
                            r += l;
                        }
                    } else {
                        r = get_limit(d, l, c, r);
                    }
                } else {
                    r = get_limit(d, l, c, r);
                }
            }
            _ => unreachable!(),
        }

        println!("{} {}", r, c);
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> Option<usize>;
    fn upper_bound(&self, x: &T) -> Option<usize>;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> Option<usize> {
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

        if self.len() > low && self[low] >= *x {
            Some(low)
        } else {
            None
        }
    }

    fn upper_bound(&self, x: &T) -> Option<usize> {
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
        if self.len() > low && self[low] > *x {
            Some(low)
        } else {
            None
        }
    }
}
