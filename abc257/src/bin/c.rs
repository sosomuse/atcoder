use proconio::{input, marker::Chars};
use superslice::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    };

    let mut children = vec![];
    let mut parents = vec![];

    for i in 0..n {
        let c = s[i];
        let v = w[i];

        match c {
            '0' => children.push(v),
            '1' => parents.push(v),
            _ => unreachable!(),
        };
    }

    if parents.len() == 0 {
        println!("{}", children.len());
        return;
    }

    children.sort();
    parents.sort();

    let mut ans = 0;

    for (i, size) in parents.iter().enumerate() {
        let correct_count = (parents.len() - i - 1) + children.lower_bound(size) + 1;
        ans = ans.max(correct_count);
    }

    println!("{}", ans);
}
