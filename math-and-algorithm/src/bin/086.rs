use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut depth = 0;

    for i in 0..n {
        let v = s[i];
        if v == '(' {
            depth += 1;
        };
        if v == ')' {
            depth -= 1;
        }

        if depth < 0 {
            println!("No");
            return;
        }
    }

    if depth == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
