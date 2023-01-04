use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        a: [usize; n],
    };

    let mut set = BTreeSet::new();
    let mut count = HashMap::new();

    let insert = |n: usize, set: &mut BTreeSet<usize>, count: &mut HashMap<usize, usize>| {
        *count.entry(n).or_insert(0) += 1;
        set.insert(n)
    };

    let remove =
        |n: usize, set: &mut BTreeSet<usize>, count: &mut HashMap<usize, usize>| match count
            .get_mut(&n)
        {
            Some(0) => {}
            Some(c) => {
                if *c == 1 {
                    *c -= 1;
                    set.remove(&n);
                } else {
                    *c -= 1;
                }
            }
            None => {}
        };

    for v in a {
        insert(v, &mut set, &mut count);
    }

    for _ in 0..m {
        let mut last = 0;

        if let Some(x) = set.range(..).next_back() {
            last = *x;
        }

        remove(last, &mut set, &mut count);
        insert(last / 2, &mut set, &mut count);
    }

    let mut sum = 0;

    for v in set {
        if let Some(x) = count.get(&v) {
            sum += x * v
        }
    }

    println!("{}", sum);
}
