use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ok = false;
    let mut count = 0;

    for i in 0..n {
        if s[i] == '|' {
            count += 1;
        }

        if count == 1 && s[i] == '*' {
            ok = true;
        }
    }

    if ok {
        println!("in");
    } else {
        println!("out");
    }
}
