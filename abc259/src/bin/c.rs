use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut cnt = 0;
    let mut prev: char = ' ';
    let mut prev_c = 1;

    for i in 0..t.len() {
        if i == 0 || i == 1 {
            if s[i] != t[i] {
                println!("No");
                return;
            }

            if prev == s[i] {
                prev_c += 1;
            } else {
                prev_c = 1;
            }

            prev = s[i];
            continue;
        }

        if i - cnt >= s.len() {
            if prev == t[i] && prev_c > 1 {
                cnt += 1;
            } else {
                println!("No");
                return;
            }

            continue;
        }

        if s[i - cnt] != t[i] {
            if prev == t[i] && prev_c > 1 {
                cnt += 1;
            } else {
                println!("No");
                return;
            }
        } else {
            if prev == s[i - cnt] {
                prev_c += 1;
            } else {
                prev_c = 1;
            }

            prev = s[i - cnt];
        }
    }

    println!("Yes");
}
