use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut is_ok = false;

    for i in 0..n {
        let v = s[i];

        if v == 'x' {
            println!("No");
            return;
        }

        if v == 'o' {
            is_ok = true;
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
