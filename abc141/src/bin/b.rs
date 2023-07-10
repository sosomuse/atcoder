use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for i in 0..s.len() {
        let c = s[i];

        if c == 'U' || c == 'D' {
            continue;
        }

        if i % 2 == 0 {
            if c == 'R' {
                continue;
            }
        } else {
            if c == 'L' {
                continue;
            }
        }

        println!("No");
        return;
    }

    println!("Yes");
}
