use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let perm = (0..s.len()).permutations(s.len()).collect_vec();
    let mut x = vec![];

    for p in perm {
        let mut y = vec![];
        for i in 0..s.len() {
            y.push(s[p[i]]);
        }
        x.push(y);
    }

    x.sort();
    x.dedup();

    println!("{}", x[k - 1].iter().join(""));
}
