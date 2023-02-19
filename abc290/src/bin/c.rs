use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    };

    a.sort();
    let uniq: Vec<_> = a.into_iter().unique().collect();

    let mut ans: usize = 0;

    for i in 0..k.min(uniq.len()) {
        if i == uniq[i] {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
