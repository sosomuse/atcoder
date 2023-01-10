use std::collections::{BTreeSet, HashMap};

use proconio::input;

#[allow(unused_assignments)]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    // 場に表示されているカード
    let mut set = BTreeSet::<usize>::new();
    // カードがどの山札にあるか
    let mut map = HashMap::<usize, usize>::new();
    // 実際のカード群
    let mut cards: Vec<Vec<usize>> = vec![];
    // 答え isizeとusizeの共用体
    let mut ans: Vec<isize> = vec![-1; n + 1];

    for i in 0..n {
        let x = p[i];
        let mut min = None;
        let mut count = 0;

        let mut range = set.range(x..);
        if let Some(&y) = range.next() {
            min = Some(y);
        }

        if let Some(min) = min {
            set.remove(&min);
            count = map[&min];
            map.insert(x, count);
        } else {
            count = cards.len();
            map.insert(x, count);
            cards.push(vec![]);
        }

        cards[count].push(x);
        set.insert(x);

        if cards[count].len() == k {
            set.remove(&x);

            for &y in &cards[count] {
                ans[y] = (i + 1) as isize;
            }
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
