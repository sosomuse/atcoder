use proconio::{input, marker::Chars};

use itertools::Itertools;

fn main() {
    input! {
        n: Chars,
    };

    let len = n.len();
    let perm = n.into_iter().permutations(len).collect::<Vec<_>>();

    let mut ans = 0;

    for p in perm {
        for v in 1..p.len() {
            let x = p[..v].iter().join("").parse::<i64>().unwrap();
            let y = p[v..].iter().join("").parse::<i64>().unwrap();

            ans = ans.max(x * y);
        }
    }

    println!("{}", ans);
}
