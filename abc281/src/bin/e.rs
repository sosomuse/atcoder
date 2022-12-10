use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    };

    let mut b = BTreeSet::new();
    let mut map = HashMap::new();
    let mut c = vec![];

    for i in 0..m {
        *map.entry(a[i]).or_insert(0) += 1;
        b.insert(a[i]);
        c.push(a[i]);
    }

    c.sort();
    let mut min = *c.iter().take(k).min().unwrap();
    let mut max = *c.iter().take(k).max().unwrap();
    let mut sum = c.iter().take(k).sum::<usize>();

    print!("{} ", sum);

    for i in 0..m - 2 {
        let v1 = &a[i];
        let v2 = &a[i + k];

        *map.entry(*v2).or_insert(0) += 1;
        b.insert(*v2);

        let mut range1 = b.range(max..);
        let mut range2 = b.range(..max);

        if *v1 <= max {
            sum -= v1;
            sum += *range1.next().unwrap();
        }

        if *v2 < max {
            sum += v2;
            sum -= max;
        }

        min = *b.iter().next().unwrap();

        print!("{} ", sum);
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
}
