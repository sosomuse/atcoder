use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    let first = s.remove(0);
    s.push(first);
    println!("{}", s.iter().collect::<String>());
}
