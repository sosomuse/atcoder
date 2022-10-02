use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();

    let mut set: BTreeSet<usize> = std::collections::BTreeSet::new();

    for i in 0..n {
        set.insert(a[i]);
    }

    let c2 = a.len() - set.len();

    let mut queue = std::collections::VecDeque::new();

    for i in set {
        queue.push_back(i);
    }

    for _ in 0..c2 {
        queue.push_back(0);
    }

    let mut ans = 0;
    let mut count = 0;

    loop {
        if ans >= queue.len() + count {
            break;
        }

        let x = queue[ans - count];
        if x == ans + 1 {
            ans += 1;
        } else {
            if ans + 2 <= queue.len() + count {
                queue.pop_back();
                queue.pop_back();

                count += 1;
                ans += 1;
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}
