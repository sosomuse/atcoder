use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    // 10進数変換
    let mut prev = n[0].to_digit(10).unwrap();

    for i in 1..n.len() {
        let now = n[i].to_digit(10).unwrap();
        if prev <= now {
            println!("No");
            return;
        }

        prev = now;
    }

    println!("Yes");
}
