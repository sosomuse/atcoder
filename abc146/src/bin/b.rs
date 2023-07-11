use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: u8,
        mut s: Chars,
    };

    for i in 0..s.len() {
        let mut c = s[i] as u8 + n;
        if c > b'Z' {
            c -= 26;
        }
        s[i] = c as char;
    }

    println!("{}", s.iter().collect::<String>());
}
