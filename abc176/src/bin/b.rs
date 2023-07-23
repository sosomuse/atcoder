use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    let mut sum = 0;
    for c in n {
        sum += c.to_digit(10).unwrap();
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
