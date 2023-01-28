use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut count = 0;

    let is_collect = |sv: char, tv: char| return sv == '?' || tv == '?' || sv == tv;

    for i in 0..t.len() {
        let sv = s[i + s.len() - t.len()];
        let tv = t[i];

        if is_collect(sv, tv) {
            count += 1;
        }
    }

    if count == t.len() {
        println!("Yes");
    } else {
        println!("No");
    }

    for i in 0..t.len() {
        let sv = s[i];
        let tv = t[i];

        if is_collect(sv, tv) {
            count += 1;
        }

        let sv2 = s[i + s.len() - t.len()];
        let tv2 = t[i];

        if is_collect(sv2, tv2) {
            count -= 1;
        }

        if count == t.len() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
