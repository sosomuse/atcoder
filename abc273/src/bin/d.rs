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

    for _ in 0..q {
        input! {
            d: char,
            l: usize,
        }

        match d {
            'L' => {
                if c == 1 {
                    println!("{} {}", r, c);
                    continue;
                }

                if let Some(y) = h_block.get(&(r - 1)) {
                    let t = y.lower_bound(&(c));

                    if t == 0 {
                        if c > l {
                            c -= l;
                        } else {
                            c = 1;
                        }

                        println!("{} {}", r, c);
                        continue;
                    }

                    if let Some(z) = y.get(t - 1) {
                        if z + l >= c {
                            c = z + 1;
                        } else {
                            c -= l
                        }
                    }
                } else {
                    if c > l {
                        c -= l;
                    } else {
                        c = 1;
                    }
                }
            }
            'R' => {
                if c == w {
                    println!("{} {}", r, c);

                    continue;
                }

                if let Some(y) = h_block.get(&(r - 1)) {
                    let t = y.lower_bound(&(c));

                    if let Some(z) = y.get(t) {
                        if c + l >= *z {
                            c = z - 1;
                        } else {
                            c += l;
                        }
                    } else {
                        if c + l > w {
                            c = w;
                        } else {
                            c += l;
                        }
                    }
                } else {
                    if c <= w - l {
                        c += l;
                    }
                }
            }
            'U' => {
                if r == 1 {
                    println!("{} {}", r, c);
                    continue;
                }

                if let Some(y) = w_block.get(&(c - 1)) {
                    let t = y.lower_bound(&(r));

                    if t == 0 {
                        if r > l {
                            r -= l;
                        } else {
                            r = 1;
                        }

                        println!("{} {}", r, c);
                        continue;
                    }

                    if let Some(z) = y.get(t - 1) {
                        if z + l >= r {
                            r = z + 1;
                        } else {
                            r -= l
                        }
                    }
                } else {
                    if r > l {
                        r -= l;
                    } else {
                        r = 1;
                    }
                }
            }
            'D' => {
                if r == h {
                    println!("{} {}", r, c);
                    continue;
                }

                if let Some(y) = w_block.get(&(c - 1)) {
                    let t = y.lower_bound(&(r));

                    if let Some(z) = y.get(t) {
                        if r + l >= *z {
                            r = z - 1;
                        } else {
                            r += l;
                        }
                    } else {
                        if r <= h - l {
                            r += l;
                        } else {
                            r = h;
                        }
                    }
                } else {
                    if r <= h - l {
                        r += l;
                    } else {
                        r = h;
                    }
                }
            }
            _ => unreachable!(),
        }

        println!("{} {}", r, c);
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
