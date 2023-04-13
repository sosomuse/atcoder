use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); m],
    };

    let mut vertical_set = vec![BTreeSet::new(); h];
    let mut horizontal_set = vec![BTreeSet::new(); w];

    for (a, b) in cd {
        vertical_set[a - 1].insert(b - 1);
        horizontal_set[b - 1].insert(a - 1);
    }

    let mut ans = 0;

    for (a, b) in ab {
        let a = a - 1;
        let b = b - 1;
    }
}
