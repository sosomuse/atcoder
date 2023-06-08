use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        s: Chars,
    };

    if k >= s.len() {
        println!("{}", s.iter().collect::<String>());
    } else {
        println!("{}...", s.iter().take(k).collect::<String>());
    }
}
