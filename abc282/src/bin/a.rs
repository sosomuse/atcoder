use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    let mut s = String::new();
    for i in 0..k {
        s.push((b'A' + i as u8) as char);
    }

    println!("{}", s);
}
