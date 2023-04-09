use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut b1 = -1;
    let mut b2 = -1;
    let mut r_count = 0;
    let mut is_k = false;

    for i in 0..s.len() {
        let c = s[i];
        if c == 'B' {
            if b1 == -1 {
                b1 = i as i32;
            } else {
                b2 = i as i32;
            }
        }

        if c == 'R' {
            r_count += 1;
        }

        if c == 'K' && r_count == 1 {
            is_k = true;
        }
    }

    if b1 % 2 != b2 % 2 && is_k {
        println!("Yes");
    } else {
        println!("No");
    }
}
