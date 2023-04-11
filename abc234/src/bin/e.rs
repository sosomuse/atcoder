use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        x: i64,
    };

    let set = solve();
    let mut range = set.range(x..);

    println!("{}", range.next().unwrap());
}

fn solve() -> BTreeSet<i64> {
    let mut set = BTreeSet::new();

    for i in 1..=9 {
        for j in -9..=8 {
            let mut s = String::new();
            let mut d = i;

            for _ in 0..18 {
                s.push_str(&d.to_string());
                set.insert(s.parse::<i64>().unwrap());
                d += j;
                if !(0 <= d && d <= 9) {
                    break;
                }
            }
        }
    }

    return set;
}
