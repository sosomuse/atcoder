use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n : usize,
        mut h : usize,
        mut ab : [(usize, usize); n]
    }

    ab.sort_by_key(|&(_, b)| Reverse(b));
    let max = ab.iter().map(|&(a, _)| a).max().unwrap();

    let mut ans = std::usize::MAX;
    for (i, (_, b)) in ab.into_iter().enumerate() {
        h = h.saturating_sub(b);
        ans = ans.min(i + 1 + (h + max - 1) / max);
    }

    println!("{}", ans);
}
