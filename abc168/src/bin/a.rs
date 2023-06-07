use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    let last = n[n.len() - 1];
    if last == '2' || last == '4' || last == '5' || last == '7' || last == '9' {
        println!("hon");
    } else if last == '0' || last == '1' || last == '6' || last == '8' {
        println!("pon");
    } else {
        println!("bon");
    }
}
