use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut count = 0;

    let is_collect = |sc: char, tc: char| return sc == '?' || tc == '?' || sc == tc;

    for i in 0..t.len() {
        let sc = s[i + s.len() - t.len()];
        let tc = t[i];

        if is_collect(sc, tc) {
            count += 1;
        }
    }

    if count == t.len() {
        println!("Yes");
    } else {
        println!("No");
    }

    for i in 0..t.len() {
        let sc = s[i];
        let tc = t[i];
        let sc_back = s[i + s.len() - t.len()];

        if is_collect(sc, tc) {
            count += 1;
        }

        if is_collect(sc_back, tc) {
            count -= 1;
        }

        if count == t.len() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
