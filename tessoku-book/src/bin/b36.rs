use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    };

    let es = s.iter().filter(|&c| *c == '1').collect::<Vec<_>>();
    let x = es.len();

    if k % 2 == x % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
