use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    if s.len() != 8 {
        println!("No");
        return;
    }

    if !s[0].is_uppercase() || !s[7].is_uppercase() {
        println!("No");
        return;
    }

    let n = s[1..7].iter().collect::<String>();
    if let Ok(m) = n.parse::<i32>() {
        if m >= 100000 && m <= 999999 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
