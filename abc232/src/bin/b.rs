use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    };

    let sub = (t[0] as i8 - s[0] as i8 + 26) % 26;

    for i in 0..s.len() {
        s[i] = (((s[i] as i8) - b'a' as i8 + sub) % 26_i8 + b'a' as i8).abs() as u8 as char;
    }

    if s.iter().collect::<String>() == t.iter().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
