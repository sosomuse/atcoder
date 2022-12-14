use std::collections::BTreeSet;

use proconio::input;

#[allow(unused_variables, unused_mut, dead_code)]

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    };

    let mut multi_set = MultiSet::new();
    let mut c = vec![];

    for i in 0..m {
        multi_set.insert(a[i]);
        c.push(a[i]);
    }

    c.sort();
    let mut max = *c.iter().take(k).max().unwrap();
    let mut sum = c.iter().take(k).sum::<usize>();

    print!("{} ", sum);

    for i in 0..m - 2 {
        let v1 = &a[i];
        let v2 = &a[i + m];

        multi_set.remove(*v1);

        // dbg!(i, v1, v2);
        // dbg!(sum, &multi_set);

        if *v1 <= max {
            sum -= *v1;
            let range1 = multi_set.set.range(..=(max, 1000000));

            if range1.count() < k {
                let mut range2 = multi_set.set.range((max + 1, 0)..);
                let (x, _) = range2.next().unwrap();
                max = *x;
                sum += max;
            }
        }

        multi_set.insert(*v2);

        if *v2 <= max {
            sum += *v2;
            sum -= max;

            let range1 = multi_set.set.range(..=(max - 1, 1000000));

            if range1.count() >= k {
                let mut range2 = multi_set.set.range(..(max - 1, 1000000));
                let (x, _) = range2.rev().next().unwrap();
                max = *x;
            }
        }

        print!("{} ", sum);
    }
}

#[derive(Clone, Debug)]
struct MultiSet {
    set: BTreeSet<(usize, usize)>,
}

impl MultiSet {
    fn new() -> Self {
        Self {
            set: BTreeSet::new(),
        }
    }
    fn insert(&mut self, x: usize) {
        let mut e = (x, 1);
        if let Some((_, c)) = self.set.range((x, 0)..).next() {
            e = (x, c + 1);
        }
        self.set.insert(e);
    }
    fn remove(&mut self, x: usize) {
        if let Some((x2, c)) = self.set.range((x, 0)..).next() {
            if x == *x2 {
                self.set.remove(&(x, *c));
            }
        }
    }
}
