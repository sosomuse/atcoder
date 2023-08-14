use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
    };

    for i in (0..n.len()).rev() {
        if n[i] == '0' {
            n.pop();
        } else {
            break;
        }
    }

    let mut ans = true;
    for i in 0..n.len() / 2 {
        if n[i] != n[n.len() - 1 - i] {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
