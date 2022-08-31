use proconio::input;

use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    };

    let cmb = (0..n).combinations(5).collect::<Vec<Vec<usize>>>();
    let mut ans: usize = 0;

    for vec in cmb.iter() {
        let prd = vec.iter().map(|&i| a[i]).product::<usize>();
        if prd % p == q {
            ans += 1;
        }
    }

    println!("{}", ans);
}
