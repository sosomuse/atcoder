use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let c = s[n - 1];

    if c == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
