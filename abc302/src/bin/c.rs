use proconio::{input, marker::Chars};

use itertools::Itertools;
fn main() {
    input! {
        n: usize,
        _: usize,
        s: [Chars; n],
    };

    let perm = (0..n).permutations(n).collect::<Vec<Vec<usize>>>();

    for p in perm {
        let mut prev = &s[p[0]];

        for i in 1..n {
            let diff = diff(prev, &s[p[i]]);
            if diff != 1 {
                break;
            }

            prev = &s[p[i]];
            if i == n - 1 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn diff(a: &Vec<char>, b: &Vec<char>) -> usize {
    let mut res = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            res += 1;
        }
    }

    res
}
