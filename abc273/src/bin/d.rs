use std::cmp::*;
use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        h: isize,
        w: isize,
        mut r: isize,
        mut c: isize,
        n: usize,
        rc: [(isize, isize); n],
        q: usize,
    };

    let mut h_block = HashMap::new();
    let mut w_block = HashMap::new();

    for (r, c) in &rc {
        h_block.entry(*r - 1).or_insert(BTreeSet::new()).insert(*c);
        w_block.entry(*c - 1).or_insert(BTreeSet::new()).insert(*r);
    }

    for _ in 0..q {
        input! {
            d: char,
            l: isize,
        }

        let def = BTreeSet::<isize>::new();

        match d {
            'L' => {
                let y = h_block.get(&(r - 1)).unwrap_or(&def);
                let mut t = y.range(..c);
                let next = t.next_back();

                c -= l;
                if let Some(wall) = next {
                    c = max(c, *wall + 1);
                } else {
                    c = max(c, 1);
                }
            }
            'R' => {
                let y = h_block.get(&(r - 1)).unwrap_or(&def);
                let mut t = y.range(c + 1..);
                let next = t.next();

                c += l;
                if let Some(wall) = next {
                    c = min(c, *wall - 1);
                } else {
                    c = min(c, h);
                }
            }
            'U' => {
                let y = w_block.get(&(c - 1)).unwrap_or(&def);
                let mut t = y.range(..r);
                let next = t.next_back();

                r -= l;
                if let Some(wall) = next {
                    r = max(r, *wall + 1);
                } else {
                    r = max(r, 1);
                }
            }
            'D' => {
                let y = w_block.get(&(c - 1)).unwrap_or(&def);
                let mut t = y.range(r + 1..);
                let next = t.next();

                r += l;

                if let Some(wall) = next {
                    r = min(r, *wall - 1);
                } else {
                    r = min(r, w);
                }
            }
            _ => unreachable!(),
        }

        println!("{} {}", r, c);
    }
}
