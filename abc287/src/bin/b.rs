use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let mut matches = vec![false; n];

    for i in 0..n {
        let v = &s[i];
        let pref = &v[3..];

        for j in 0..m {
            let ti = &t[j];

            if pref == ti {
                matches[i] = true;
                break;
            }
        }
    }

    println!("{}", matches.iter().filter(|&s| *s).count());
}
