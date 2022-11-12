use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let x = s
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let is_same = x[0] == x[1] && x[1] == x[2] && x[2] == x[3];
    let is_continue = f(x[0]) == x[1] && f(x[1]) == x[2] && f(x[2]) == x[3];

    if is_same || is_continue {
        println!("Weak");
    } else {
        println!("Strong");
    }
}

fn f(n: u32) -> u32 {
    (n + 1) % 10
}
