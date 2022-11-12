use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    // 場に表示されているカード
    let mut set = BTreeSet::new();
    // 場に表示されているカードがどこに属しているか
    let mut map = HashMap::new();
    // 実際のカード群
    let mut vec: Vec<Vec<usize>> = vec![vec![]; n];
    // 山札のカウント
    let mut count = 0;
    // どのカード群がいつ食べられたか
    let mut ans = HashMap::new();

    for i in 0..n {
        let v = p[i];
        let mut z = 0;

        if let Some(x) = set.range(v..).next() {
            z = *x;
            let y = map.get(x).unwrap();
            map.insert(v, *y);
            set.insert(v);
        } else {
            vec[count].push(v);

            if k != 1 {
                map.insert(v, count);
                set.insert(v);
            } else {
                ans.insert(count, k);
            }

            count += 1;
        }

        set.remove(&z);
    }

    println!("{}", n);
}
